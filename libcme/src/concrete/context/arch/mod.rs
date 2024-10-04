//! arch module
//! 
//! architecture-specific state implementation for CallOther

use thiserror::Error;
use dyn_clone::{DynClone, clone_trait_object};

pub mod cm3;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error(transparent)]
    CortexM3(#[from] cm3::Error),
}


