//! peripheral.rs
//! 
//! peripheral definitions that can be mapped into contexts
pub mod dummy;
pub mod channel;

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
    #[error("time error: {0}")]
    Time(anyhow::Error),
}

impl Error {
    pub fn state<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::State(anyhow::Error::new(e))
    }

    pub fn time<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Time(anyhow::Error::new(e))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    // Generic(String),
    /// enable external interrupt with given interrupt number
    /// which is always its exception number - 16
    EnableInterrupt { int_num: u32 },
    /// disable external interrupt with given interrupt number
    /// which is always its exception number - 16
    DisableInterrupt { int_num: u32 },
    /// fire an interrupt
    FireInterrupt { int_num: u32 },
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
    fn tick(&mut self) -> Result<Option<Event>, Error> { Ok(None) }
}
clone_trait_object!(PeripheralState);

/// peripheral wrapper struct
/// 
/// defining a new peripheral constitutes passing an internal state generic
#[derive(Clone)]
pub struct Peripheral {
    state: Box<dyn PeripheralState>,
}

impl Peripheral {
    pub fn new_with(state: Box<dyn PeripheralState>) -> Self {
        Self { state }
    }

    pub fn base_address(&self) -> Address {
        self.state.base_address()
    }

    pub fn size(&self) -> u64 {
        self.state.size()
    }

    pub fn tick(&mut self) -> Result<Option<Event>, Error> {
        self.state.tick()
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
