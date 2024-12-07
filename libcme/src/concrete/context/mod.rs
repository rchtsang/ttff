//! context.rs
//! 
//! evaluation contex trait

pub mod arch;

use std::fmt;
use std::ops::Range;

use thiserror::Error;
use flagset::flags;

use fugue_ir::{Address, VarnodeData};
use fugue_ir::disassembly::PCodeData;
use fugue_ir::error::Error as IRError;
use fugue_core::ir::Location;
use fugue_core::language::{Language, LanguageBuilderError};
use fugue_core::eval::fixed_state::FixedStateError;
use fugue_bv::BitVec;

use crate::peripheral;
use super::*;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("lifter error: {0}")]
    Lift(String),
    #[error("language builder error: {0}")]
    LangBuilder(String),
    #[error(transparent)]
    Arch(#[from] arch::Error),
    #[error("peripheral error: {0}")]
    Peripheral(String),
    #[error("invalid address: {0:#x?}")]
    AddressInvalid(BitVec),
    #[error("address not lifted: {0:#x?}")]
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

impl From<LanguageBuilderError> for Error {
    fn from(value: LanguageBuilderError) -> Self {
        Self::LangBuilder(format!("{value:?}"))
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

impl From<peripheral::Error> for Error {
    fn from(value: peripheral::Error) -> Self {
        Self::Peripheral(format!("{value:?}"))
    }
}

flags! {
    pub enum Permission: u8 {
        R = 0x04,
        W = 0x02,
        E = 0x01,
        RO = 0x04,
        WO = 0x02,
        RW = (Permission::R | Permission::W).bits(),
        RE = (Permission::R | Permission::E).bits(),
        WE = (Permission::W | Permission::E).bits(),
        RWE = (Permission::R | Permission::W | Permission::E).bits(),
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Alignment {
    Byte = 0b001,
    Half = 0b010,
    Word = 0b100,
    Even = 0b110,
    Any  = 0b111,
}

/// context request
/// 
/// evaluator interacts with context using message passing pattern
/// 
/// this should allow for easier observability on the context side since
/// all of these can can be handled in a single function and observers can be
/// dispatched from a central location without having to litter them everywhere
#[derive(Debug)]
pub enum CtxRequest<'a> {
    Fetch { address: Address },
    Read { vnd: &'a VarnodeData },
    Write { vnd: &'a VarnodeData, val: &'a BitVec },
    Load { address: Address, size: usize },
    Store { address: Address, val: &'a BitVec },
    LoadBytes { address: Address, dst: &'a mut [u8] },
    StoreBytes { address: Address, bytes: &'a [u8] },
    ReadPc,
    WritePc { address: Address },
    ReadSp,
    WriteSp { address: Address },
    CallOther { output: Option<&'a VarnodeData>, inputs: &'a [VarnodeData] },
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
    LoadBytes { result: Result<(), Error> },
    StoreBytes { result: Result<(), Error> },
    ReadPc { result: Result<Address, Error> },
    WritePc { result: Result<(), Error> },
    ReadSp { result: Result<Address, Error> },
    WriteSp { result: Result<(), Error> },
    CallOther { result: Result<Option<Location>, Error> },
}


/// context trait
/// 
/// an architecture emulation context implementation should implement this trait to keep the
/// actual evaluator architecture agnostic
pub trait Context<'irb>: fmt::Debug {

    fn lang(&self) -> &Language;

    fn fmt_pcodeop(&self, pcodeop: &PCodeData) -> String {
        crate::utils::fmt_pcodeop(pcodeop, self.lang().translator(), Some(true))
    }

    /// evaluate request in context and return a response
    /// 
    /// a struct that implements context must implement a single request function that 
    /// handles every basic CtxRequest enum variant.
    /// forcing all types of context interactions through this single request function 
    /// makes implementing observability things a bit easier.
    fn request(&mut self, req: CtxRequest) -> CtxResponse<'irb>;

    /// fetch the lifted instruction at the given address
    fn fetch(&mut self, address: impl Into<Address>) -> LiftResult<'irb> {
        let address = address.into();
        self.request(CtxRequest::Fetch { address }).into()
    }

    /// read a varnode
    fn read(&mut self, vnd: &VarnodeData) -> Result<BitVec, Error> {
        self.request(CtxRequest::Read { vnd }).into()
    }

    /// write a varnode
    fn write(&mut self, vnd: &VarnodeData, val: &BitVec) -> Result<(), Error> {
        self.request(CtxRequest::Write { vnd, val }).into()
    }

    /// read the current pc address
    fn read_pc(&mut self) -> Result<Address, Error> {
        self.request(CtxRequest::ReadPc).into()
    }

    /// write an address to the pc
    fn write_pc(&mut self, address: impl Into<Address>) -> Result<(), Error> {
        let address = address.into();
        self.request(CtxRequest::WritePc { address }).into()
    }

    /// read the current stack pointer address
    fn read_sp(&mut self) -> Result<Address, Error> {
        self.request(CtxRequest::ReadSp).into()
    }

    /// write an address the the active stack pointer
    fn write_sp(&mut self, address: impl Into<Address>) -> Result<(), Error> {
        let address = address.into();
        self.request(CtxRequest::WriteSp { address }).into()
    }

    /// load a value from mapped memory
    fn load(&mut self, address: impl Into<Address>, size: usize) -> Result<BitVec, Error> {
        let address = address.into();
        self.request(CtxRequest::Load { address, size }).into()
    }

    /// store a value in mapped memory
    fn store(&mut self, address: impl Into<Address>, val: &BitVec) -> Result<(), Error> {
        let address = address.into();
        self.request(CtxRequest::Store { address, val }).into()
    }

    /// load bytes from mapped memory into a destination buffer
    fn load_bytes(&mut self, address: impl Into<Address>, dst: &mut [u8]) -> Result<(), Error> {
        let address = address.into();
        self.request(CtxRequest::LoadBytes { address, dst }).into()
    }

    /// store bytes from a source buffer into mapped memory
    fn store_bytes<'a>(&mut self, address: impl Into<Address>, bytes: &'a [u8]) -> Result<(), Error> {
        let address = address.into();
        self.request(CtxRequest::StoreBytes { address, bytes }).into()
    }

    /// call a user-defined pcode operation
    /// 
    /// on succes, returns a Location (from an address) if the userop performs a branch.
    /// no implemented userops currently branch at all,
    /// but this is left as is for future support if necessry.
    fn userop(&mut self, output: Option<&VarnodeData>, inputs: &[VarnodeData]) -> Result<Option<Location>, Error> {
        self.request(CtxRequest::CallOther { output, inputs }).into()
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
            CtxResponse::WritePc { result } => { result }
            CtxResponse::WriteSp { result } => { result }
            CtxResponse::LoadBytes { result } => { result }
            CtxResponse::StoreBytes { result } => { result }
            _ => { panic!("expected Store or Write response! got: {self:?}") }
        }
    }
}

impl<'irb> Into<Result<Address, Error>> for CtxResponse<'irb> {
    fn into(self) -> Result<Address, Error> {
        match self {
            CtxResponse::ReadPc { result } => { result }
            CtxResponse::ReadSp { result } => { result }
            _ => { panic!("expected ReadPc response! got: {self:?}") }
        }
    }
}

impl<'irb> Into<Result<Option<Location>, Error>> for CtxResponse<'irb> {
    fn into(self) -> Result<Option<Location>, Error> {
        match self {
            CtxResponse::CallOther { result } => { result }
            _ => { panic!("expected CallOther response! got: {self:?}") }
        }
    }
}