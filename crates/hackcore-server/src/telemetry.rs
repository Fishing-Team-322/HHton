use tracing_subscriber::{fmt, EnvFilter};

/// Initialise the global tracing subscriber using an environment filter.
pub fn init() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_target(true)
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);
}
