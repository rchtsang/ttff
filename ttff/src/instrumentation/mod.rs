//! instrumentation module
//! 
//! implements various architecture and analysis plugins
//! for the harness

pub mod covmap;
pub mod mem;
pub mod hc;
pub mod csbc;
pub mod cmplog;
pub mod ttrace;

pub use covmap::CovMap;
pub use hc::HcPlugin;
pub use csbc::CsbcPlugin;
pub use mem::{MemCallback, MemInterceptPlugin};
pub use ttrace::TaintTracePlugin;