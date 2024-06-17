use std::{path::Path, str::FromStr};

use tracing::Level;
use tracing_subscriber::{
    filter::Directive, fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt,
    EnvFilter,
};

pub fn init_logger(logs_directory: impl AsRef<Path>) {
    let log_file = tracing_appender::rolling::daily(logs_directory, "shadowverse-utils.log")
        .with_min_level(Level::INFO);
    let log_file_layer = tracing_subscriber::fmt::Layer::new().with_writer(log_file);

    let console_log = tracing_subscriber::fmt::Layer::new()
        .with_ansi(true)
        .with_writer(
            std::io::stderr
                .with_min_level(Level::WARN)
                .or_else(std::io::stdout.with_min_level(Level::DEBUG)),
        );

    let _ = tracing_subscriber::registry()
        .with(console_log)
        .with(log_file_layer)
        .with(
            EnvFilter::from_default_env().add_directive(
                Directive::from_str(&format!(
                    "{}=DEBUG",
                    env!("CARGO_PKG_NAME").replace("-", "_"),
                ))
                .unwrap(),
            ),
        )
        .try_init();
}
