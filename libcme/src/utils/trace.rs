//! trace.rs
//! 
//! tracing utilities

pub use tracing::{
    self,
    instrument,
    event,
    span,
    Level,
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

pub fn compact_logger() -> FmtSubscriber<DefaultFields, Format<Compact>> {
    fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .with_target(true)
    .finish()
}

pub fn compact_dbg_logger() -> FmtSubscriber<DefaultFields, Format<Compact>> {
    fmt()
    .compact()
    .with_file(true)
    .with_line_number(true)
    .with_target(true)
    .with_max_level(Level::DEBUG)
    .finish()
}