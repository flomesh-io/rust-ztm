#[allow(dead_code)] // used in tests
pub fn init_logger(level: &str) {
    let level = match level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt().with_max_level(level).finish(),
    )
    .unwrap();
    tracing::info!("logger initialized with level: {}", level);
}
