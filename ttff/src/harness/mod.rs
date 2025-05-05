//! harness module
//! 
//! a libafl-compliant harness modules for emulation
//! with libcme dft
use thiserror::Error;

/// harness error
#[derive(Debug, Error)]
pub enum Error {
    #[error("error loading input")]
    Input,
}

pub mod sc;