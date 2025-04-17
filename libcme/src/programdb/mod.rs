//! programdb module
//! 
use std::fmt;
use std::sync::Arc;

use thiserror::Error;
use parking_lot::RwLock;
use nohash::IntMap;

use fugue_core::prelude::*;
use fugue_ir::disassembly::{ Opcode, IRBuilderArena };

use crate::types::*;
use crate::utils::*;
use crate::backend::Backend;

type TranslationCache<'irb> = IntMap<u64, LiftResult<'irb>>;

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
}


impl<'irb> ProgramDB<'irb> {

    pub fn new_with(lang: Language, arena: &'irb IRBuilderArena) -> Self {
        let cache = Arc::new(RwLock::new(TranslationCache::default()));

        Self { lang, cache, arena }
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
        
        let mut branch = false;
        while !branch {
            let address = base + offset as u64;

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

        // maybe return something here at some point?
    }
}

impl<'irb> fmt::Debug for ProgramDB<'irb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arch = self.lang.translator().architecture();
        write!(f, "ProgramDB {{ {arch} }}")
    }
}