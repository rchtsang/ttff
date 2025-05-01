//! exception.rs
//! 
//! manage interrupt and exception state

use super::*;


#[allow(unused)]
/// state for nested vector interrupt controller
#[derive(Default, Debug, Clone)]
pub struct ExceptionState {
    /// sorted set of enabled exceptions
    pub(crate) enabled: Vec<ExceptionType>,
    /// sorted queue of pending exceptions, updated on
    /// any priority change or insertion/removal
    pub(crate) pending: Vec<ExceptionType>,
    /// a stack of active exceptions
    pub(crate) active: Vec<ExceptionType>,
}

impl ExceptionState {
    pub fn active(&self) -> &[ExceptionType] {
        &self.active
    }

    pub fn pending(&self) -> &[ExceptionType] {
        &self.pending
    }

    pub fn enabled(&self) -> &[ExceptionType] {
        &self.enabled
    }
}