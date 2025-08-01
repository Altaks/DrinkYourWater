use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging_system() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(true)
                .with_level(true)
                .with_target(false)
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO),
        )
        .init();
}
