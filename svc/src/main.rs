use cogs_shared::app::AppError;
use cogs_svc::server::{self, ServerState, SvcConfig, init_logging, init_router};
use config::{Config, Environment};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() {
    init_logging();

    dotenvy::dotenv().unwrap();
    let config = Config::builder()
        .add_source(Environment::with_prefix("COGS_SVC").try_parsing(true))
        .build()
        .unwrap();

    let cfg: SvcConfig = config.try_deserialize().unwrap();

    log::info!("Connecting to database ...");
    let dbcp = server::db_pool_init().await.expect("Failed to connect to the database!");
    log::info!("Connected to database.");

    let state = ServerState::new(Arc::new(dbcp.clone()));

    match state
        .user_mgmt
        .register_admin_user("Admin".into(), "admin@example.com".into(), "admin".into(), "admin".into())
        .await
    {
        Ok(_) => log::info!("Self-registered the admin user."),
        Err(e) => {
            if let AppError::AlreadyExists(_) = e {
                // It's fine if the admin user already exists.
            } else {
                log::error!("Failed to self-register the admin user: {}", e);
                return;
            }
        }
    }

    let web_api_router = init_router(&dbcp).await.with_state(state);

    log::info!("Listening on http://{}", cfg.listenaddress);
    let listener = tokio::net::TcpListener::bind(&cfg.listenaddress)
        .await
        .expect(format!("Failed to bind to address {}", cfg.listenaddress).as_str());

    axum::serve(listener, web_api_router.into_make_service())
        .with_graceful_shutdown(shutdown_signal(dbcp))
        .await
        .unwrap();
}

async fn shutdown_signal(dbcp: Pool<Postgres>) {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    log::info!("Shutting down ...");

    dbcp.close().await;
    log::info!("Database connection closed.");
}
