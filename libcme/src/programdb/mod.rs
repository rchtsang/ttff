//! programdb module
//! 
use std::fmt;
use std::sync::Arc;

use thiserror::Error;
use parking_lot::RwLock;

// use fugue_core::prelude::*;
use fugue_ir::disassembly::{ Opcode, IRBuilderArena };

use crate::types::*;
use crate::utils::*;
use crate::backend::Backend;

mod cfg;
pub use cfg::*;
mod types;
pub use types::*;

/// programdb errors
#[derive(Error, Debug)]
pub enum Error {
    
}


/// A ProgramDB should serve as an intermediary between the evaluator
/// and the context for the purposes of fetching and lifting instructions.
/// All lifted instructions should be stored in the ProgramDB, so that 
/// separate analysis can be conducted without the evaluator context,
/// independently of whether execution has taint tracking enabled or not.
pub struct ProgramDB<'irb> {
    pub(crate) lang: Language,
    pub(crate) cache: Arc<RwLock<TranslationCache<'irb>>>,
    pub(crate) arena: &'irb IRBuilderArena,
    pub(crate) cfg: CFGraph<'irb>,
}


impl<'irb> ProgramDB<'irb> {

    pub fn new_with(
        lang: Language,
        arena: &'irb IRBuilderArena,
    ) -> Self {
        let cache = Arc::new(RwLock::new(TranslationCache::default()));
        let cfg = CFGraph::new_with(arena.inner());

        Self { lang, cache, arena, cfg }
    }

    pub fn fetch(&mut self, address: Address, backend: &impl Backend) -> LiftResult<'irb> {
        let address: Address = (address.offset() & !1).into();

        if !self.cache.read().contains_key(&address.offset()) {
            self._lift_block(address, backend);
        }

        self.cache.read()
            .get(&address.offset())
            .ok_or(LiftError::AddressNotLifted(address.into()))?
            .clone()
    }
}

impl<'irb> ProgramDB<'irb> {
    #[instrument]
    fn _lift_block(&mut self,
        address: impl Into<Address> + fmt::Debug,
        backend: &impl Backend,
    ) {
        let base = address.into();
        let mut offset = 0usize;

        let mut insns = vec![];
        let mut successors = vec![];
        let mut flow = Flow::from(FlowType::Fall);
        
        let mut branch = false;
        while !branch {
            let address = base + offset as u64;
            insns.push(address);

            let fetch_result = backend.fetch(&address, self.arena);
            if fetch_result.is_err() {
                // read failed
                self.cache.write().insert(address.offset(), fetch_result);
                break;
            }
            let insn = fetch_result.unwrap();
            let pcode = &insn.pcode;
            
            offset += pcode.len();

            debug!("{:#x?}", pcode);

            if let Some(last_op) = pcode.operations.last() {
                match last_op.opcode {
                    Opcode::Branch
                    | Opcode::CBranch
                    | Opcode::IBranch
                    | Opcode::Call
                    | Opcode::ICall
                    | Opcode::Return
                    | Opcode::CallOther => {
                        // usually we can tell if the last opcode is branching
                        let loc = _absolute_loc(
                            address.into(),
                            last_op.inputs[0],
                            (pcode.operations.len() - 1) as u32,
                        );
                        flow = FlowType::from(last_op.opcode).target(loc);
                        branch = true;
                    },
                    _ => {
                        // otherwise we need to check if the pc gets written to
                        // this may never happen in pcode semantics but idk for sure.
                        // we leave it commented out for now b/c it probably doesn't matter and better performance
                        // if it turns out it's possible we will uncomment and kill this comment
                        // branch = pcode.operations.iter().any(|pcodedata| {
                        //     if let Some(vnd) = pcodedata.output {
                        //         vnd == self.pc
                        //     } else {
                        //         false
                        //     }
                        // });
                    },
                }
            }

            self.cache.write().insert(address.offset(), Ok(insn));
        }

        // get the known block successors
        let flowtype = flow.flowtype;
        match flowtype {
            FlowType::Branch
            | FlowType::Call => {
                let dest = flow.target.unwrap().address().offset();
                successors.push((flowtype, dest));
            }
            FlowType::CBranch => {
                let dest = flow.target.unwrap().address().offset();
                successors.push((flowtype, dest));
                successors.push((FlowType::Fall, offset as u64));
            }
            FlowType::IBranch
            | FlowType::ICall => {
                // successors are unknown until runtime
                // since blocks are prefetched, we cannot read the target from
                // the backend in this function.
            }
            _ => {
                successors.push((FlowType::Fall, offset as u64));
            }
        }

        let range = base.offset()..(offset as u64);
        let insns = insns.into_iter().map(|addr| addr.offset());
        let block = Block::new_in(self.arena.inner(), range, insns, successors);

        match self.cfg.add_block(block, None) {
            Ok(_) => { () }
            Err(err) => { panic!("unexpected error: {err:?}") }
        }
        // maybe return something here at some point?
    }
}

impl<'irb> fmt::Debug for ProgramDB<'irb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arch = self.lang.translator().architecture();
        write!(f, "ProgramDB {{ {arch} }}")
    }
}