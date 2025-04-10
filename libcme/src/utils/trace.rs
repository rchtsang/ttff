//! trace.rs
//! 
//! tracing utilities

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
    fmt,
    fmt::format::{DefaultFields, Format, Compact},
    FmtSubscriber,
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