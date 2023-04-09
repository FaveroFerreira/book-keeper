use anyhow::Context;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

pub fn init_tracing() -> anyhow::Result<WorkerGuard> {
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_span_list(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(non_blocking_writer)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("failed to init tracing subscriber")?;

    Ok(guard)
}
