//! peripheral.rs
//! 
//! peripheral definitions that can be mapped into contexts
pub mod dummy;

use std::ops::Range;

use anyhow;
use thiserror::Error;
use dyn_clone::{DynClone, clone_trait_object};
use fugue_core::prelude::*;


#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    State(anyhow::Error),
}

impl Error {
    pub fn state<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::State(anyhow::Error::new(e))
    }
}


/// peripheral state trait
/// 
/// the peripheral struct is a wrapper for objects that implement
/// this trait.
pub trait PeripheralState: DynClone {
    fn read_bytes(&mut self, address: &Address, dst: &mut [u8]) -> Result<(), Error>;
    fn write_bytes(&mut self, address: &Address, src: &[u8]) -> Result<(), Error>;
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
    pub fn new_with(range: Range<Address>, state: Box<dyn PeripheralState>) -> Self {
        Self { range, state }
    }

    pub fn read_bytes(&mut self,
        address: &Address,
        dst: &mut [u8],
    ) -> Result<(), Error> {
        self.state.read_bytes(address, dst)
    }

    pub fn write_bytes(&mut self,
        address: &Address,
        src: &[u8],
    ) -> Result<(), Error> {
        self.state.write_bytes(address, src)
    }

}
