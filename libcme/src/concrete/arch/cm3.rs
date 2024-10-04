//! cm3.rs
//! 
//! cortex-m3 emulation context
//! 
//! implement the minimum necessary peripherals to emulate a cortex-m3
//! microprocessor.
use std::sync::Arc;
use std::ops::Range;

use thiserror::Error;
use nohash::IntMap;
use iset::IntervalMap;
use parking_lot::RwLock;

use fugue_bv::BitVec;
use fugue_ir::{
    Address, VarnodeData,
    convention::Convention,
    error::Error as IRError,
    disassembly::{IRBuilderArena, Opcode},
};
use fugue_core::language::Language;
use fugue_core::eval::fixed_state::{FixedState, FixedStateError};

use crate::concrete;
use crate::concrete::{
    types::*,
    context,
    context::{CtxRequest, CtxResponse},
};

pub use crate::concrete::context::Context as ContextTrait;
pub type TranslationCache<'irb> = IntMap<u64, LiftResult<'irb>>;


#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error(transparent)]
    Context(#[from] context::Error),
}

#[derive(Clone, Copy, Debug)]
enum MapIx {
    Mem(usize),
    Mmio(usize),
}

/// the cortex-m3 execution context
/// 
/// a context must contain all state information needed for execution, the evaluator should not require state
#[derive(Clone)]
pub struct Context<'irb> {
    lang: Language,
    pc: VarnodeData,
    apsr: VarnodeData, // cpsr in ghidra sla
    // armv7m xPSR is a combination of APSR, IPSR, and EPSR
    // and is not defined as part of the ghidra sleigh spec.
    // hence we must handle this manually
    xpsr: BitVec,

    irb: &'irb IRBuilderArena,
    regs: FixedState,
    tmps: FixedState,
    mmap: IntervalMap<Address, MapIx>,
    mem: Vec<FixedState>,
    // mmio: Vec<???> // todo: add peripheral models
    cache: Arc<RwLock<TranslationCache<'irb>>>,
}


impl<'irb> Context<'irb> {

    fn lift_block(&mut self,
        address: impl Into<Address>,
        // irb: &'irb IRBuilderArena,
    ) {
        let mut lifter = self.lang.lifter();
        let base = address.into();
        let mut offset = 0usize;
        
        loop {
            let address = base + offset as u64;

            
        }

        todo!()
    }
}


// private implementations
impl<'irb> Context<'irb> {
    fn _fetch(&self, address: impl Into<Address>) -> LiftResult<'irb> {
        let address = address.into();
        self.cache.read()
            .get(&address.offset())
            .ok_or(context::Error::AddressNotLifted(address.clone()))?
            .clone()
    }

    fn _get_mapped_region(&self, address: Address) -> Result<(Range<Address>, MapIx), context::Error> {
        let mut overlaps = self.mmap.overlap(address.clone());
        let (range, val) = overlaps.next()
            .ok_or(context::Error::Unmapped(address.clone()))?;
        if let Some((other_range, _)) = overlaps.next() {
            return Err(context::Error::MapConflict(range, other_range));
        }
        Ok((range, val.clone()))
    }

    fn _mem_view_bytes(&self, address: impl AsRef<Address>, size: usize) -> Result<&[u8], context::Error> {
        let address = address.as_ref();
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.view_bytes(offset, size)
                    .map_err(context::Error::from)
            }
            MapIx::Mmio(_idx) => {
                panic!("mmio peripherals can't implement view_bytes due to their send/receive data model")
            }
        }
    }

    fn _mem_view_bytes_mut(&mut self, address: impl AsRef<Address>, size: usize) -> Result<&mut [u8], context::Error> {
        let address = address.as_ref();
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get_mut(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.view_bytes_mut(offset, size)
                    .map_err(context::Error::from)
            }
            MapIx::Mmio(_idx) => {
                panic!("mmio peripherals can't implement view_bytes due to their send/receive data model")
            }
        }
    }

    fn _map_read_bytes(&mut self, address: impl AsRef<Address>, dst: &mut [u8]) -> Result<(), context::Error> {
        let address = address.as_ref();
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.read_bytes(offset, dst)
                    .map_err(context::Error::from)
            }
            MapIx::Mmio(idx) => {
                todo!("yet to implement peripherals (have a peripheral struct with generic fields/callbacks)")
            }
        }
    }

    fn _map_write_bytes(&mut self, address: impl AsRef<Address>, src: &[u8]) -> Result<(), context::Error> {
        let address = address.as_ref();
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get_mut(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.write_bytes(offset, src)
                    .map_err(context::Error::from)
            }
            MapIx::Mmio(idx) => {
                todo!("yet to implement peripherals")
            }
        }
    }

    fn _map_read_val(&mut self, address: impl AsRef<Address>, size: usize) -> Result<BitVec, context::Error> {
        let big_endian = self.lang.translator().is_big_endian();
        let mut dst = vec![0u8; size];
        let view = self._map_read_bytes(address, &mut dst)?;

        if big_endian {
            Ok(BitVec::from_be_bytes(&dst))
        } else {
            Ok(BitVec::from_le_bytes(&dst))
        }
    }

    fn _map_write_val(&mut self, address: impl AsRef<Address>, val: &BitVec) -> Result<(), context::Error> {
        let size = val.bytes();
        let mut src = vec![0u8; size];
        if self.lang.translator().is_big_endian() {
            val.to_be_bytes(&mut src);
        } else {
            val.to_le_bytes(&mut src);
        }

        self._map_write_bytes(address, &src)
    }

    fn _read_vnd(&mut self, vnd: &VarnodeData) -> Result<BitVec, context::Error> {
        let spc = vnd.space();
        if spc.is_constant() {
            todo!()
        } else if spc.is_register() {
            todo!()
        } else if spc.is_unique() {
            todo!()
        } else if spc.is_default() {
            todo!()
        } else {
            panic!("read from {spc:?} unsupported")
        }
    }

    fn _write_vnd(&mut self, vnd: &VarnodeData, val: &BitVec) -> Result<(), context::Error> {
        let spc = vnd.space();
        if spc.is_register() {
            todo!()
        } else if spc.is_unique() {
            todo!()
        } else if spc.is_constant() {
            todo!()
        } else if spc.is_default() {
            todo!()
        } else {
            panic!("read from {spc:?} unsupported")
        }
    }
}


impl<'irb> context::Context<'irb> for Context<'irb> {
    fn request(&mut self, req: CtxRequest) -> CtxResponse<'irb> {
        match req {
            CtxRequest::Fetch { address } => {
                CtxResponse::Fetch { result: self._fetch(address) }
            }
            CtxRequest::Read { vnd } => {
                CtxResponse::Read { result: self._read_vnd(vnd) }
            }
            CtxRequest::Write { vnd, val } => {
                CtxResponse::Write { result: self._write_vnd(vnd, val) }
            }
            CtxRequest::Load { address, size } => {
                CtxResponse::Load { result: self._map_read_val(address, size) }
            }
            CtxRequest::Store { address, val } => {
                CtxResponse::Store { result: self._map_write_val(address, val) }
            }
        }
    }
}