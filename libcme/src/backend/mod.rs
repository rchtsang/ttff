//! backend.rs
//! 
//! architecture backends must implement this trait to be used in context
use std::sync::Arc;
use std::ops::Range;
use std::fmt;

use fugue_core::eval::fixed_state::FixedStateError;
use thiserror::Error;

use fugue_ir::{Address, VarnodeData};
use fugue_ir::disassembly::PCodeData;
use fugue_core::ir::Location;
use fugue_core::language::{Language, LanguageBuilderError};
use fugue_bv::BitVec;

use crate::types::{LiftResult, LiftError};
use crate::peripheral;

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


pub trait Backend<'irb>: fmt::Debug {

    fn lang(&self) -> &Language;

    fn fmt_pcodeop(&self, pcodeop: &PCodeData) -> String {
        crate::utils::fmt_pcodeop(pcodeop, self.lang().translator(), Some(true))
    }

    /// fetch the lifted instruction at the given address
    fn fetch(&mut self, address: &Address) -> LiftResult<'irb>;

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