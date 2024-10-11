//! arch module
//! 
//! architecture-specific state implementation for CallOther

use thiserror::Error;

pub mod armv7m;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error(transparent)]
    Armv7M(#[from] armv7m::Error),
}


