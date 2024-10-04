//! context.rs
//! 
//! evaluation contex trait

pub mod arch;

use std::sync::Arc;
use std::ops::Range;

use thiserror::Error;

use fugue_ir::{ Address, VarnodeData };
use fugue_ir::{
    disassembly::IRBuilderArena,
    error::Error as IRError,
};
use fugue_core::eval::fixed_state::FixedStateError;
use fugue_bv::BitVec;

use super::types::*;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("lifter error: {0}")]
    Lift(String),
    #[error(transparent)]
    Arch(#[from] arch::Error),
    #[error("address not lifted: {0:x?}")]
    AddressNotLifted(Address),
    #[error("address in unmapped memory: {0}")]
    Unmapped(Address),
    #[error("mapped regions conflict: {0:#x?} and {1:#x?}")]
    MapConflict(Range<Address>, Range<Address>),
    #[error("out of bounds fixedstate read: [{offset:#x}; {size}]")]
    OOBRead { offset: usize, size: usize },
    #[error("out of bounds fixedstate write: [{offset:#x}; {size}]")]
    OOBWrite { offset: usize, size: usize },
}

impl From<IRError> for Error {
    fn from(value: IRError) -> Self {
        Self::Lift(format!("{value:?}"))
    }
}

impl From<FixedStateError> for Error {
    fn from(value: FixedStateError) -> Self {
        match value {
            FixedStateError::OOBRead { offset, size } => Error::OOBRead { offset, size },
            FixedStateError::OOBWrite { offset, size } => Error::OOBWrite { offset, size },
        }
    }
}


/// context request
/// 
/// evaluator interacts with context using message passing pattern
/// 
/// this should allow for easier observability on the context side since
/// all of these can can be handled in a single function and observers can be
/// dispatched from a central location without having to litter them everywhere
#[derive(Clone, PartialEq, Eq)]
pub enum CtxRequest<'a> {
    Fetch { address: Address },
    Read { vnd: &'a VarnodeData },
    Write { vnd: &'a VarnodeData, val: &'a BitVec },
    Load { address: Address, size: usize },
    Store { address: Address, val: &'a BitVec },
}

/// context request response
/// 
/// contains the result of the context request
#[derive(Debug)]
pub enum CtxResponse<'irb> {
    Fetch { result: LiftResult<'irb> },
    Read { result: Result<BitVec, Error> },
    Write { result: Result<(), Error> },
    Load { result: Result<BitVec, Error> },
    Store { result: Result<(), Error> },
}


/// context trait
/// 
/// an architecture emulation context implementation should implement this trait to keep the
/// actual evaluator architecture agnostic
pub trait Context<'irb> {
    /// evaluate request in context and return a response
    /// 
    /// a struct that implements context must implement a single request function that 
    /// handles every basic CtxRequest enum variant.
    /// forcing all types of context interactions through this single request function 
    /// makes implementing observability things a bit easier.
    fn request(&mut self, req: CtxRequest) -> CtxResponse<'irb>;

    fn fetch(&mut self, address: Address) -> LiftResult<'irb> {
        self.request(CtxRequest::Fetch { address }).into()
    }

    fn read(&mut self, vnd: &VarnodeData) -> Result<BitVec, Error> {
        self.request(CtxRequest::Read { vnd }).into()
    }

    fn write(&mut self, vnd: &VarnodeData, val: &BitVec) -> Result<(), Error> {
        self.request(CtxRequest::Write { vnd, val }).into()
    }

    fn load(&mut self, address: Address, size: usize) -> Result<BitVec, Error> {
        self.request(CtxRequest::Load { address, size }).into()
    }

    fn store(&mut self, address: Address, val: &BitVec) -> Result<(), Error> {
        self.request(CtxRequest::Store { address, val }).into()
    }
}


impl<'irb> Into<LiftResult<'irb>> for CtxResponse<'irb> {
    fn into(self) -> LiftResult<'irb> {
        match self {
            CtxResponse::Fetch { result } => { result }
            _ => { panic!("expected Fetch response! got: {self:?}") }
        }
    }
}

impl<'irb> Into<Result<BitVec, Error>> for CtxResponse<'irb> {
    fn into(self) -> Result<BitVec, Error> {
        match self {
            CtxResponse::Load { result } => { result }
            CtxResponse::Read { result } => { result }
            _ => { panic!("expected Load or Read response! got: {self:?}") }
        }
    }
}

impl<'irb> Into<Result<(), Error>> for CtxResponse<'irb> {
    fn into(self) -> Result<(), Error> {
        match self {
            CtxResponse::Store { result } => { result }
            CtxResponse::Write { result } => { result }
            _ => { panic!("expected Store or Write response! got: {self:?}") }
        }
    }
}