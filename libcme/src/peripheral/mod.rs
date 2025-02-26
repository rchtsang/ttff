//! peripheral.rs
//! 
//! peripheral definitions that can be mapped into contexts
pub mod dummy;
pub mod models;
pub use models::*;

use std::ops::Range;
use std::collections::VecDeque;

use anyhow;
use thiserror::Error;
use dyn_clone::{DynClone, clone_trait_object};
use fugue_core::prelude::*;


#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    State(anyhow::Error),
    #[error("invalid peripheral register access @ {0:#x?}")]
    InvalidPeripheralReg(Address),
}

impl Error {
    pub fn state<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::State(anyhow::Error::new(e))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Generic(String),
}

/// peripheral state trait
/// 
/// the peripheral struct is a wrapper for objects that implement
/// this trait.
pub trait PeripheralState: DynClone {
    fn base_address(&self) -> Address;
    fn size(&self) -> u64;
    fn read_bytes(&mut self, address: &Address, dst: &mut [u8], events: &mut VecDeque<Event>) -> Result<(), Error>;
    fn write_bytes(&mut self, address: &Address, src: &[u8], events: &mut VecDeque<Event>) -> Result<(), Error>;
}
clone_trait_object!(PeripheralState);

/// peripheral wrapper struct
/// 
/// defining a new peripheral constitutes passing an internal state generic
#[derive(Clone)]
pub struct Peripheral {
    pub range: Range<Address>,
    state: Box<dyn PeripheralState>,
}

impl Peripheral {
    pub fn new_with(state: Box<dyn PeripheralState>) -> Self {
        let start = state.base_address();
        let end = start + state.size();
        let range = start..end;
        Self { range, state }
    }

    pub fn read_bytes(&mut self,
        address: &Address,
        dst: &mut [u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), Error> {
        self.state.read_bytes(address, dst, events)
    }

    pub fn write_bytes(&mut self,
        address: &Address,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), Error> {
        self.state.write_bytes(address, src, events)
    }

}
