//! error.rs
//! 
//! error types for concrete errors

use thiserror::Error;

use super::context;
use super::eval;

/// concrete evaluator errors
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Context(#[from] context::Error),
    #[error(transparent)]
    Eval(#[from] eval::Error),
}