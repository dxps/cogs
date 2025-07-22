pub fn init_logging() {
    use log::LevelFilter::{Debug, Info, Warn};

    simple_logger::SimpleLogger::new()
        .with_module_level("cogs_svc", Debug)
        .with_module_level("sqlx", Warn)
        .with_module_level("tungstenite", Info)
        .with_module_level("tokio_tungstenite", Info)
        .with_module_level("axum_session", Info)
        .with_module_level("axum_session_auth", Warn)
        .with_module_level("axum", Info)
        .with_module_level("warnings", Warn)
        .with_module_level("tracing", Warn)
        .init()
        .unwrap();
}
