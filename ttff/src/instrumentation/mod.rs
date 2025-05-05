//! instrumentation module
//! 
//! implements various architecture and analysis plugins
//! for the harness

pub mod covmap;
pub mod hc;
pub mod csbc;
pub mod cmplog;

pub use covmap::CovMap;