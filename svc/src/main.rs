use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::AuthConfig;
use axum_session_sqlx::SessionPgSessionStore;
use cogs_svc::{
    AppError,
    domain::model::Id,
    server::{self, AuthSessionLayer, ServerState, SvcConfig, create_router, init_logging},
};
use config::{Config, Environment};
use std::sync::Arc;
use tokio::signal;

pub use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    init_logging();

    dotenvy::dotenv().unwrap();
    let config = Config::builder()
        .add_source(
            Environment::with_prefix("COGS_SVC").try_parsing(true), // .separator("_")
                                                                    // .list_separator(" "),
        )
        .build()
        .unwrap();

    let cfg: SvcConfig = config.try_deserialize().unwrap();

    log::debug!("Loaded cfg: {cfg:#?}");

    log::info!("Connecting to database ...");
    let dbcp = server::db_pool_init()
        .await
        .expect("Failed to connect to the database!");
    log::info!("Connected to database.");

    let session_config = SessionConfig::default()
        .with_session_name("cogs_session")
        .with_table_name("user_sessions")
        .with_lifetime(chrono::Duration::hours(24))
        .with_purge_database_update(chrono::Duration::minutes(5));
    let session_store = SessionPgSessionStore::new(Some(dbcp.clone().into()), session_config)
        .await
        .unwrap();
    let auth_config = AuthConfig::<Id>::default().with_anonymous_user_id(Some(0.into()));

    let state = ServerState::new(Arc::new(dbcp.clone()));

    match state
        .user_mgmt
        .register_admin_user(
            "Admin".into(),
            "admin@example.com".into(),
            "admin".into(),
            "admin".into(),
        )
        .await
    {
        Ok(_) => log::info!("Self-registered the admin user."),
        Err(e) => {
            if let AppError::AlreadyExists(_) = e {
                // It's fine if the admin user already exists.
            } else {
                log::error!("Failed to self-register the admin user: {}", e);
            }
        }
    }

    let router = create_router(state)
        .layer(AuthSessionLayer::new(Some(dbcp)).with_config(auth_config))
        .layer(SessionLayer::new(session_store));

    log::info!("Listening on http://{}", cfg.listenaddress);
    let listener = tokio::net::TcpListener::bind(&cfg.listenaddress)
        .await
        .expect(format!("Failed to bind to address {}", cfg.listenaddress).as_str());

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
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
    // TODO: Close the db connection pool and free any other resources.
}
