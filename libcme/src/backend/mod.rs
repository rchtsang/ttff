//! backend.rs
//! 
//! architecture backends must implement this trait to be used in context
use std::sync::Arc;
use std::ops::Range;
use std::fmt;

use thiserror::Error;
use dyn_clone::{DynClone, clone_trait_object};

use fugue_ir::{Address, VarnodeData};
use fugue_ir::disassembly::{IRBuilderArena, PCodeData};
use fugue_core::ir::Location;
use fugue_core::language::{Language, LanguageBuilderError};
use fugue_core::eval::fixed_state::FixedStateError;
use fugue_bv::BitVec;

use crate::types::*;
use crate::peripheral::{self, Peripheral};

pub mod armv7m;

/// backend errors
#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error(transparent)]
    Lift(Arc<fugue_ir::error::Error>),
    #[error(transparent)]
    State(Arc<FixedStateError>),
    #[error("language builder error: {0}")]
    LangBuilder(Arc<LanguageBuilderError>),
    #[error("peripheral error: {0}")]
    Peripheral(Arc<peripheral::Error>),
    #[error("invalid address: {0:#x?}")]
    AddressInvalid(BitVec),
    #[error("address not lifted: {0:#x?}")]
    AddressNotLifted(Address),
    #[error("address in unmapped memory: {0}")]
    Unmapped(Address),
    #[error("mapped regions conflict: {0:#x?} and {1:#x?}")]
    MapConflict(Range<Address>, Range<Address>),
    // #[error("out of bounds fixedstate read: [{offset:#x}; {size}]")]
    // OOBRead { offset: usize, size: usize },
    // #[error("out of bounds fixedstate write: [{offset:#x}; {size}]")]
    // OOBWrite { offset: usize, size: usize },
    #[error("{0} error: {1:?}")]
    Arch(&'static str, Arc<anyhow::Error>),
}


pub trait Backend: fmt::Debug + DynClone {

    fn lang(&self) -> &Language;

    fn fmt_pcodeop(&self, pcodeop: &PCodeData) -> String {
        crate::utils::fmt_pcodeop(pcodeop, self.lang().translator(), Some(true))
    }

    /// get context's current thread
    fn current_thread(&self) -> EmuThread;

    /// checks if isr is pending. if one is, performs preempt and returns the new thread context
    fn do_isr_preempt(&self) -> Option<EmuThread>;

    /// checks if isr is returning. if it is, performs return and returns the new thread context
    fn do_isr_return(&self) -> Option<EmuThread>;

    /// initialize a memory region in the context's memory map
    fn map_mem(&mut self, base: &Address, size: usize) -> Result<(), Error>;

    /// initialize a peripheral in the context's memory map
    fn map_mmio(&mut self, peripheral: Peripheral) -> Result<(), Error>;

    /// fetch the instruction bytes at the given address
    fn fetch<'irb>(&self, address: &Address, arena: &'irb IRBuilderArena) -> LiftResult<'irb>;

    /// read a varnode
    fn read(&mut self, vnd: &VarnodeData) -> Result<BitVec, Error>;

    /// write a varnode
    fn write(&mut self, vnd: &VarnodeData, val: &BitVec) -> Result<(), Error>;

    /// read the current pc address
    fn read_pc(&mut self) -> Result<Address, Error>;

    /// write an address to the pc
    fn write_pc(&mut self, address: &Address) -> Result<(), Error>;

    /// read the current stack pointer address
    fn read_sp(&mut self) -> Result<Address, Error>;

    /// write an address the the active stack pointer
    fn write_sp(&mut self, address: &Address) -> Result<(), Error>;

    /// load a value from mapped memory
    fn load(&mut self, address: &Address, size: usize) -> Result<BitVec, Error>;

    /// store a value in mapped memory
    fn store(&mut self, address: &Address, val: &BitVec) -> Result<(), Error>;

    /// load bytes from mapped memory into a destination buffer
    fn load_bytes(&mut self, address: &Address, dst: &mut [u8]) -> Result<(), Error>;

    /// store bytes from a source buffer into mapped memory
    fn store_bytes<'a>(&mut self, address: &Address, bytes: &'a [u8]) -> Result<(), Error>;

    /// call a user-defined pcode operation
    /// 
    /// on succes, returns a Location (from an address) if the userop performs a branch.
    /// no implemented userops currently branch at all,
    /// but this is left as is for future support if necessry.
    fn userop(&mut self, output: Option<&VarnodeData>, inputs: &[VarnodeData]) -> Result<Option<Location>, Error>;
}
clone_trait_object!(Backend);


impl From<fugue_ir::error::Error> for Error {
    fn from(err: fugue_ir::error::Error) -> Self {
        Self::Lift(Arc::new(err))
    }
}

impl From<FixedStateError> for Error {
    fn from(err: FixedStateError) -> Self {
        Self::State(Arc::new(err))
    }
}

impl From<LanguageBuilderError> for Error {
    fn from(err: LanguageBuilderError) -> Self {
        Self::LangBuilder(Arc::new(err))
    }
}

impl From<peripheral::Error> for Error {
    fn from(err: peripheral::Error) -> Self {
        Self::Peripheral(Arc::new(err))
    }
}

impl Into<Arc<anyhow::Error>> for Error {
    fn into(self) -> Arc<anyhow::Error> {
        Arc::new(anyhow::Error::from(self))
    }
}

impl From<Error> for LiftError {
    fn from(err: Error) -> Self {
        LiftError::Backend(anyhow::Error::from(err))
    }
}

impl From<Error> for Arc<LiftError> {
    fn from(err: Error) -> Self {
        Arc::new(LiftError::from(err))
    }
}

impl<'backend> Backend for Box<dyn Backend + 'backend> {
    fn lang(&self) -> &Language { (**self).lang() }
    fn fmt_pcodeop(&self, pcodeop: &PCodeData) -> String { (**self).fmt_pcodeop(pcodeop) }
    fn current_thread(&self) -> EmuThread { (**self).current_thread() }
    fn do_isr_preempt(&self) -> Option<EmuThread> { (**self).do_isr_preempt() }
    fn do_isr_return(&self) -> Option<EmuThread> { (**self).do_isr_return() }
    fn map_mem(&mut self, base: &Address, size: usize) -> Result<(), Error> { (**self).map_mem(base, size) }
    fn map_mmio(&mut self, peripheral: Peripheral) -> Result<(), Error> { (**self).map_mmio(peripheral) }
    fn fetch<'irb>(&self, address: &Address, arena: &'irb IRBuilderArena) -> LiftResult<'irb> { (**self).fetch(address, arena) }
    fn read(&mut self, vnd: &VarnodeData) -> Result<BitVec, Error> { (**self).read(vnd) }
    fn write(&mut self, vnd: &VarnodeData, val: &BitVec) -> Result<(), Error> { (**self).write(vnd, val) }
    fn read_pc(&mut self) -> Result<Address, Error> { (**self).read_pc() }
    fn write_pc(&mut self, address: &Address) -> Result<(), Error> { (**self).write_pc(address) }
    fn read_sp(&mut self) -> Result<Address, Error> { (**self).read_sp() }
    fn write_sp(&mut self, address: &Address) -> Result<(), Error> { (**self).write_sp(address) }
    fn load(&mut self, address: &Address, size: usize) -> Result<BitVec, Error> { (**self).load(address, size) }
    fn store(&mut self, address: &Address, val: &BitVec) -> Result<(), Error> { (**self).store(address, val) }
    fn load_bytes(&mut self, address: &Address, dst: &mut [u8]) -> Result<(), Error> { (**self).load_bytes(address, dst) }
    fn store_bytes<'a>(&mut self, address: &Address, bytes: &'a [u8]) -> Result<(), Error> { (**self).store_bytes(address, bytes) }
    fn userop(&mut self, output: Option<&VarnodeData>, inputs: &[VarnodeData]) -> Result<Option<Location>, Error> { (**self).userop(output, inputs) }
}