//! trace.rs
//! 
//! tracing utilities
use std::fs::OpenOptions;
use tracing::{level_filters::LevelFilter, Subscriber};
pub use tracing::{
    self,
    instrument,
    event,
    span,
    Level,
    subscriber::set_global_default,
    debug,
    error,
    info,
    warn,
};
use tracing_subscriber::{
    prelude::*,
    fmt,
    fmt::format::{Compact, DefaultFields, Format},
    FmtSubscriber,
    Registry,
};

/// configure tracing with a compact logger
/// ```
/// use tracing::subscriber::set_global_default;
/// set_global_default(compact_logger())
///     .expect("failed to set global default");
/// ```
pub fn compact_logger() -> FmtSubscriber<DefaultFields, Format<Compact>> {
    fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .with_target(true)
    .finish()
}

/// configure tracing with a compact debug level logger
/// ```
/// use tracing::set_global_default;
/// set_global_default(compact_dbg_logger())
///     .expect("failed to set global default");
/// ```
pub fn compact_dbg_logger() -> FmtSubscriber<DefaultFields, Format<Compact>> {
    fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .with_target(true)
    .with_max_level(Level::DEBUG)
    .finish()
}

/// configure tracing to output to stdout and file
pub fn compact_dbg_file_logger(path: &str) -> impl Subscriber {
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    Registry::default()
        .with(
            fmt::layer()
                .compact()
                .with_file(true)
                .with_line_number(true)
                .with_target(true)
        )
        .with(
            fmt::layer()
                .with_ansi(true)
                .with_writer(log_file)
                .compact()
                .with_file(true)
                .with_line_number(true)
                .with_target(true)
        )
        .with(LevelFilter::from_level(Level::DEBUG))
}

/// configure tracing to output to stdout and file
pub fn compact_file_logger(path: &str, level: Level) -> impl Subscriber {
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    Registry::default()
        .with(
            fmt::layer()
                .compact()
                .with_file(true)
                .with_line_number(true)
                .with_target(true)
        )
        .with(
            fmt::layer()
                .with_ansi(true)
                .with_writer(log_file)
                .compact()
                .with_file(true)
                .with_line_number(true)
                .with_target(true)
        )
        .with(LevelFilter::from_level(level))
}