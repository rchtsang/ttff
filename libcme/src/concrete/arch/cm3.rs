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
    #[error("address in unmapped memory: {0}")]
    Unmapped(Address),
    #[error("mapped regions conflict: {0:#x?} and {1:#x?}")]
    MapConflict(Range<Address>, Range<Address>),
    #[error("out of bounds fixedstate read: [{offset:#x}; {size}]")]
    OOBRead { offset: usize, size: usize },
    #[error("out of bounds fixedstate write: [{offset:#x}; {size}]")]
    OOBWrite { offset: usize, size: usize },
}

impl From<FixedStateError> for Error {
    fn from(value: FixedStateError) -> Self {
        match value {
            FixedStateError::OOBRead { offset, size } => Error::OOBRead { offset, size },
            FixedStateError::OOBWrite { offset, size } => Error::OOBWrite { offset, size },
        }
    }
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
    cache: Arc<RwLock<TranslationCache<'irb>>>,
}


impl<'irb> Context<'irb> {

    fn _fetch(&self, address: impl Into<Address>) -> LiftResult<'irb> {
        let address = address.into();
        self.cache.read()
            .get(&address.offset())
            .ok_or(context::Error::AddressNotLifted(address.clone()))?
            .clone()
    }

    fn _view_bytes(&self, address: impl AsRef<Address>, size: usize) -> Result<&[u8], Error> {
        let address = address.as_ref();
        let mut overlaps = self.mmap.overlap(address.clone());
        let (range, val) = overlaps.next()
            .ok_or(Error::Unmapped(address.clone()))?;
        if let Some((other_range, _)) = overlaps.next() {
            return Err(Error::MapConflict(range, other_range));
        }
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get(*idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.view_bytes(offset, size)
                    .map_err(Error::from)
            }
            MapIx::Mmio(idx) => {
                panic!("mmio peripherals can't implement view_bytes due to their send/receive data model")
            }
        }
    }

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

impl<'irb> context::Context<'irb> for Context<'irb> {
    fn request(&mut self, req: CtxRequest) -> CtxResponse<'irb> {
        todo!()
    }
}