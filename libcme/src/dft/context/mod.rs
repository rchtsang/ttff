//! context.rs
//! 
//! evaluation contex trait

// pub mod arch;

// use std::fmt;
use std::ops::Range;

use thiserror::Error;

use fugue_ir::{Address, VarnodeData};
use fugue_ir::disassembly::{IRBuilderArena, PCodeData};
use fugue_ir::error::Error as IRError;
use fugue_core::ir::Location;
use fugue_core::language::{Language, LanguageBuilderError};
use fugue_core::eval::fixed_state::FixedStateError;
use fugue_bv::BitVec;

use crate::types::*;
use crate::peripheral::{self, Peripheral};
use crate::utils;

use crate::backend::{self, Backend};
use super::tag::{self, Tag};

mod shadow;
use shadow::ShadowState;
mod plugin;
use plugin::*;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("lifter error: {0}")]
    Lift(String),
    #[error("language builder error: {0}")]
    LangBuilder(String),
    // #[error(transparent)]
    // Arch(#[from] arch::Error),
    #[error(transparent)]
    Backend(#[from] backend::Error),
    #[error(transparent)]
    Shadow(#[from] shadow::Error),
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

/// context request
/// 
/// evaluator interacts with context using message passing pattern
/// 
/// this should allow for easier observability on the context side since
/// all of these can can be handled in a single function and observers can be
/// dispatched from a central location without having to litter them everywhere
#[derive(Debug)]
pub enum CtxRequest<'a> {
    Fetch { address: Address, arena: &'a IRBuilderArena },
    Read { vnd: &'a VarnodeData },
    Write { vnd: &'a VarnodeData, val: &'a BitVec, tag: &'a Tag },
    Load { address: Address, size: usize },
    Store { address: Address, val: &'a BitVec, tag: &'a Tag },
    LoadBytes { address: Address, dst: &'a mut [u8] },
    StoreBytes { address: Address, bytes: &'a [u8], tag: &'a Tag },
    ReadPc,
    WritePc { address: Address, tag: &'a Tag },
    ReadSp,
    WriteSp { address: Address, tag: &'a Tag },
    CallOther { output: Option<&'a VarnodeData>, inputs: &'a [VarnodeData] },
}

/// context request response
/// 
/// contains the result of the context request
#[derive(Debug)]
pub enum CtxResponse<'irb> {
    Fetch { result: LiftResult<'irb> },
    Read { result: Result<(BitVec, Tag), Error> },
    Write { result: Result<(), Error> },
    Load { result: Result<(BitVec, Tag), Error> },
    Store { result: Result<(), Error> },
    LoadBytes { result: Result<Tag, Error> },
    StoreBytes { result: Result<(), Error> },
    ReadPc { result: Result<(Address, Tag), Error> },
    WritePc { result: Result<(), Error> },
    ReadSp { result: Result<(Address, Tag), Error> },
    WriteSp { result: Result<(), Error> },
    CallOther { result: Result<Option<Location>, Error> },
}

/// context trait
/// 
/// an architecture emulation context implementation should implement this trait to keep the
/// actual evaluator architecture agnostic
#[derive(Debug)]
pub struct Context<'backend> {
    /// the architecture-specific backend for this context
    backend: Box<dyn Backend + 'backend>,
    shadow: ShadowState,
    arch_plugin: Box<dyn ArchPlugin + 'backend>,
}


impl<'backend> Context<'backend> {

    pub fn new_with(backend: Box<dyn Backend + 'backend>) -> Self {
        let shadow = ShadowState::new_with(backend.lang().clone());
        let arch = backend.lang().translator().architecture();
        let arch_plugin = plugin_from(arch);
        Self { backend, shadow, arch_plugin }
    }

    pub fn lang(&self) -> &Language {
        self.backend.lang()
    }

    pub fn backend(&self) -> & (impl Backend + use<'backend>) {
        &self.backend
    }

    pub fn fmt_pcodeop(&self, pcodeop: &PCodeData) -> String {
        self.backend.fmt_pcodeop(pcodeop)
    }

    pub fn fmt_inputs(&mut self, pcodeop: &PCodeData) -> Result<String, Error> {
        let mut result = String::new();
        for input in pcodeop.inputs.iter() {
            if input.space().is_constant() {
                continue;
            }
            let (bv, tag) = self.read(input)?;
            let t = self.backend.lang().translator();
            result.push_str(&format!("{}=({:#x}, {}), ", utils::fmt_vnd(input, t, Some(true)), bv, tag));
        }
        Ok(result)
    }

    pub fn map_mem(
        &mut self,
        base: impl Into<Address>,
        size: usize,
    ) -> Result<(), Error> {
        let base = base.into();
        self.backend.map_mem(&base, size)?;
        self.shadow.map_mem(base, size, None)?;
        Ok(())
    }

    pub fn map_mmio(
        &mut self,
        peripheral: Peripheral,
        tag: Option<Tag>,
    ) -> Result<(), Error> {
        let base = peripheral.base_address();
        let size = peripheral.size() as usize;
        self.shadow.map_mem(base, size, tag)?;
        self.backend.map_mmio(peripheral)?;
        Ok(())
    }
}

impl<'backend> Context<'backend> {
    // interaction implementations

    /// tick processor clock
    pub fn tick(&mut self) -> Result<(), Error> {
        self.backend.tick().map_err(Error::from)
    }

    /// check for and apply thread switches
    /// returns the thread switch if taken, as well as the tag 
    /// of the target address
    pub fn maybe_thread_switch(&mut self) -> Result<Option<(backend::ThreadSwitch, Tag)>, Error> {
        let Some(ctx) = self.backend.maybe_thread_switch() else {
            return Ok(None)
        };
        let tag = self.arch_plugin.maybe_thread_switch(&mut self.shadow, &ctx)?;
        Ok(Some((ctx, tag)))
    }

    /// process any pending backend events
    pub fn process_events(&mut self) -> Result<(), Error> {
        self.backend.process_events().map_err(Error::from)
    }

    /// fetch the lifted instruction at the given address
    pub fn fetch<'irb>(&mut self, address: impl Into<Address>, arena: &'irb IRBuilderArena) -> LiftResult<'irb> {
        let address = address.into();
        self.request(CtxRequest::Fetch { address, arena }).into()
    }

    /// read a varnode
    pub fn read(&mut self, vnd: &VarnodeData) -> Result<(BitVec, Tag), Error> {
        self.request(CtxRequest::Read { vnd }).into()
    }

    /// write a varnode
    pub fn write(&mut self, vnd: &VarnodeData, val: &BitVec, tag: &Tag) -> Result<(), Error> {
        self.request(CtxRequest::Write { vnd, val, tag }).into()
    }

    /// read the current pc address
    pub fn read_pc(&mut self) -> Result<(Address, Tag), Error> {
        self.request(CtxRequest::ReadPc).into()
    }

    /// write an address to the pc
    pub fn write_pc(&mut self, address: impl Into<Address>, tag: &Tag) -> Result<(), Error> {
        let address = address.into();
        self.request(CtxRequest::WritePc { address, tag }).into()
    }

    /// read the current stack pointer address
    pub fn read_sp(&mut self) -> Result<(Address, Tag), Error> {
        self.request(CtxRequest::ReadSp).into()
    }

    /// write an address the the active stack pointer
    pub fn write_sp(&mut self, address: impl Into<Address>, tag: &Tag) -> Result<(), Error> {
        let address = address.into();
        self.request(CtxRequest::WriteSp { address, tag }).into()
    }

    /// load a value from mapped memory
    pub fn load(&mut self, address: impl Into<Address>, size: usize) -> Result<(BitVec, Tag), Error> {
        let address = address.into();
        self.request(CtxRequest::Load { address, size }).into()
    }

    /// store a value in mapped memory
    pub fn store(&mut self, address: impl Into<Address>, val: &BitVec, tag: &Tag) -> Result<(), Error> {
        let address = address.into();
        self.request(CtxRequest::Store { address, val, tag }).into()
    }

    /// load bytes from mapped memory into a destination buffer
    pub fn load_bytes(&mut self, address: impl Into<Address>, dst: &mut [u8]) -> Result<Tag, Error> {
        let address = address.into();
        self.request(CtxRequest::LoadBytes { address, dst }).into()
    }

    /// store bytes from a source buffer into mapped memory
    pub fn store_bytes<'a>(&mut self, address: impl Into<Address>, bytes: &'a [u8], tag: &Tag) -> Result<(), Error> {
        let address = address.into();
        self.request(CtxRequest::StoreBytes { address, bytes, tag }).into()
    }

    pub fn view_tags(&mut self, address: impl Into<Address>, size: usize) -> Result<&[Tag], Error> {
        let address = address.into();
        self.shadow.view_mem_tags(&address, size)
            .map_err(Error::from)
    }

    pub fn write_tags(&mut self, address: impl Into<Address>, size: usize, tag: impl Into<Tag>) -> Result<(), Error> {
        let address = address.into();
        let tag = tag.into();
        self.shadow.write_mem_tags(&address, size, &tag)
            .map_err(Error::from)
    }

    /// call a user-defined pcode operation
    /// 
    /// on succes, returns a Location (from an address) if the userop performs a branch.
    /// no implemented userops currently branch at all,
    /// but this is left as is for future support if necessry.
    pub(crate) fn userop(&mut self, output: Option<&VarnodeData>, inputs: &[VarnodeData]) -> Result<Option<Location>, Error> {
        // this is kept as a request despite being more of an evaluator 
        // operation because userops more often than not have 
        // side-effects that modify the system state
        self.request(CtxRequest::CallOther { output, inputs }).into()
    }
}

impl<'backend> Context<'backend> {
    fn request<'irb>(&mut self, req: CtxRequest<'irb>) -> CtxResponse<'irb> {
        match req {
            CtxRequest::Fetch { address, arena } => {
                CtxResponse::Fetch { result: self.backend.fetch(&address, arena) }
            }
            CtxRequest::Read { vnd } => {
                let backend_result = self.backend.read(vnd);
                if let Err(err) = backend_result {
                    return CtxResponse::Read { result: Err(err.into()) };
                }
                let bv = backend_result.unwrap();
                let shadow_result = self.shadow.read_tag(vnd);
                if let Err(err) = shadow_result {
                    return CtxResponse::Read { result: Err(err.into()) };
                };
                let tag = shadow_result.unwrap();
                CtxResponse::Read { result: Ok((bv, tag)) }
            }
            CtxRequest::Write { vnd, val, tag } => {
                let backend_result = self.backend.write(vnd, val);
                if let Err(err) = backend_result {
                    return CtxResponse::Write { result: Err(err.into()) }
                }
                let shadow_result = self.shadow.write_tag(vnd, tag);
                if let Err(err) = shadow_result {
                    return CtxResponse::Write { result: Err(err.into()) }
                }
                CtxResponse::Write { result: Ok(()) }
            }
            CtxRequest::Load { address, size } => {
                let backend_result = self.backend.load(&address, size);
                if let Err(err) = backend_result {
                    return CtxResponse::Load { result: Err(err.into()) }
                }
                let bv = backend_result.unwrap();
                let shadow_result = self.shadow.read_mem_tags(address, size);
                if let Err(err) = shadow_result {
                    return CtxResponse::Load { result: Err(err.into()) }
                }
                let tag = shadow_result.unwrap();
                CtxResponse::Load { result: Ok((bv, tag)) }
            }
            CtxRequest::Store { address, val, tag } => {
                let backend_result = self.backend.store(&address, val);
                if let Err(err) = backend_result {
                    return CtxResponse::Store { result: Err(err.into()) }
                }
                let shadow_result = self.shadow.write_mem_tags(address, val.bytes(), tag);
                if let Err(err) = shadow_result {
                    return CtxResponse::Store { result: Err(err.into()) }
                }
                CtxResponse::Store { result: Ok(()) }
            }
            CtxRequest::LoadBytes { address, dst } => {
                let backend_result = self.backend.load_bytes(&address, dst);
                if let Err(err) = backend_result {
                    return CtxResponse::LoadBytes { result: Err(err.into()) }
                }
                let shadow_result = self.shadow.read_mem_tags(address, dst.len());
                if let Err(err) = shadow_result {
                    return CtxResponse::LoadBytes { result: Err(err.into()) }
                }
                let tag = shadow_result.unwrap();
                CtxResponse::LoadBytes { result: Ok(tag) }
            }
            CtxRequest::StoreBytes { address, bytes, tag } => {
                let backend_result = self.backend.store_bytes(&address, bytes);
                if let Err(err) = backend_result {
                    return CtxResponse::StoreBytes { result: Err(err.into()) }
                }
                let shadow_result = self.shadow.write_mem_tags(address, bytes.len(), tag);
                if let Err(err) = shadow_result {
                    return CtxResponse::StoreBytes { result: Err(err.into()) }
                }
                CtxResponse::StoreBytes { result: Ok(()) }
            }
            CtxRequest::ReadPc => {
                let backend_result = self.backend.read_pc();
                if let Err(err) = backend_result {
                    return CtxResponse::ReadPc { result: Err(err.into()) }
                }
                let address = backend_result.unwrap();
                let shadow_result = self.shadow.get_pc_tag();
                if let Err(err) = shadow_result {
                    return CtxResponse::ReadPc { result: Err(err.into()) }
                }
                let tag = shadow_result.unwrap();
                CtxResponse::ReadPc { result: Ok((address, tag)) }
            }
            CtxRequest::WritePc { address, tag } => {
                let backend_result = self.backend.write_pc(&address);
                if let Err(err) = backend_result {
                    return CtxResponse::WritePc { result: Err(err.into()) }
                }
                let shadow_result = self.shadow.set_pc_tag(tag);
                if let Err(err) = shadow_result {
                    return CtxResponse::WritePc { result: Err(err.into()) }
                }
                CtxResponse::WritePc { result: Ok(()) }
            }
            CtxRequest::ReadSp => {
                let backend_result = self.backend.read_sp();
                if let Err(err) = backend_result {
                    return CtxResponse::ReadSp { result: Err(err.into()) }
                }
                let address = backend_result.unwrap();
                let shadow_result = self.shadow.get_sp_tag();
                if let Err(err) = shadow_result {
                    return CtxResponse::ReadSp { result: Err(err.into()) }
                }
                let tag = shadow_result.unwrap();
                CtxResponse::ReadSp { result: Ok((address, tag)) }
            }
            CtxRequest::WriteSp { address, tag } => {
                let backend_result = self.backend.write_sp(&address);
                if let Err(err) = backend_result {
                    return CtxResponse::WriteSp { result: Err(err.into()) }
                }
                let shadow_result = self.shadow.set_sp_tag(tag);
                if let Err(err) = shadow_result {
                    return CtxResponse::WriteSp { result: Err(err.into()) }
                }
                CtxResponse::WriteSp { result: Ok(()) }
            }
            CtxRequest::CallOther { output, inputs } => {
                CtxResponse::CallOther { 
                    result: self.backend.userop(output, inputs)
                        .map_err(|e| e.into())
                }
            }
        }
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

impl<'irb> Into<Result<(BitVec, Tag), Error>> for CtxResponse<'irb> {
    fn into(self) -> Result<(BitVec, Tag), Error> {
        match self {
            CtxResponse::Load { result } => { result.map_err(|e| e.into()) }
            CtxResponse::Read { result } => { result.map_err(|e| e.into()) }
            _ => { panic!("expected Load or Read response! got: {self:?}") }
        }
    }
}

impl <'irb> Into<Result<Tag, Error>> for CtxResponse<'irb> {
    fn into(self) -> Result<Tag, Error> {
        match self {
            CtxResponse::LoadBytes { result } => { result.map_err(|e| e.into()) }
            _ => { panic!("expected LoadBytes response! got: {self:?}") }
        }
    }
}

impl<'irb> Into<Result<(), Error>> for CtxResponse<'irb> {
    fn into(self) -> Result<(), Error> {
        match self {
            CtxResponse::Store { result } => { result.map_err(|e| e.into()) }
            CtxResponse::Write { result } => { result.map_err(|e| e.into()) }
            CtxResponse::WritePc { result } => { result.map_err(|e| e.into()) }
            CtxResponse::WriteSp { result } => { result.map_err(|e| e.into()) }
            CtxResponse::StoreBytes { result } => { result.map_err(|e| e.into()) }
            _ => { panic!("expected Store or Write response! got: {self:?}") }
        }
    }
}

impl<'irb> Into<Result<(Address, Tag), Error>> for CtxResponse<'irb> {
    fn into(self) -> Result<(Address, Tag), Error> {
        match self {
            CtxResponse::ReadPc { result } => { result.map_err(|e| e.into()) }
            CtxResponse::ReadSp { result } => { result.map_err(|e| e.into()) }
            _ => { panic!("expected ReadPc response! got: {self:?}") }
        }
    }
}

impl<'irb> Into<Result<Option<Location>, Error>> for CtxResponse<'irb> {
    fn into(self) -> Result<Option<Location>, Error> {
        match self {
            CtxResponse::CallOther { result } => { result.map_err(|e| e.into()) }
            _ => { panic!("expected CallOther response! got: {self:?}") }
        }
    }
}