//! programdb module
//! 
use std::fmt;
use std::sync::Arc;

use thiserror::Error;
use parking_lot::RwLock;
use nohash::IntMap;
use ahash::{AHashSet, AHashMap};

use fugue_ir::disassembly::{ Opcode, IRBuilderArena };

use crate::types::*;
use crate::utils::*;
use crate::backend::Backend;

pub mod types;
pub use types::*;

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
    pub last_points: AHashMap<EmuThread, ProgramPoint>,
    pub last_thread: Option<EmuThread>,
    pub cfg: CFGraph,
    pub entrypoints: AHashSet<ProgramPoint>,
}


impl<'irb> ProgramDB<'irb> {

    pub fn new_with(lang: Language, arena: &'irb IRBuilderArena) -> Self {
        let cache = Arc::new(RwLock::new(TranslationCache::default()));
        let cfg = CFGraph::default();
        let entrypoints = AHashSet::default();
        let last_points = AHashMap::default();
        let last_thread = None;

        Self { lang, cache, arena, cfg, entrypoints, last_points, last_thread }
    }

    pub fn fetch(
        &mut self,
        address: Address,
        backend: &impl Backend,
        flow: FlowType,
        prev_address: Option<Address>,
    ) -> LiftResult<'irb> {
        let address: Address = (address.offset() & !1).into();

        if !self.cache.read().contains_key(&address.offset()) {
            self._lift_block(address, backend, flow, prev_address);
        }

        let thread = backend.current_thread();
        self.last_points.insert(thread, ProgramPoint { thread, address });
        self.last_thread = Some(thread);

        self._get_lift_result(address)
    }
}

impl<'irb> ProgramDB<'irb> {
    #[instrument]
    fn _lift_block(&mut self,
        address: impl Into<Address> + fmt::Debug,
        backend: &impl Backend,
        flow: FlowType,
        prev_address: Option<Address>,
    ) {
        let base = address.into();
        let mut offset = 0usize;
        
        // on calls and returns, want to add dummy nodes to the control flow graph
        let mut prev_node = match flow {
            FlowType::Entry => {
                let thread = backend.current_thread();
                CFNode::ThreadEntry { id: thread }
            }
            FlowType::Call(_loc)
            | FlowType::ICall(_loc) => {
                let src = CFNode::Insn { address: prev_address.unwrap().offset() };
                let dst = CFNode::FunctionEntry { start: base.offset() };
                let edge = CFEdge { src, dst, flow: Some(flow) };
                self.cfg.add_edge(src, dst, edge);
                dst
            }
            FlowType::Return(_loc) => {
                let prev_address = prev_address.unwrap().offset();
                let src = CFNode::Insn { address: prev_address };
                let dst = CFNode::FunctionExit { exit: prev_address };
                let edge = CFEdge { src, dst, flow: Some(flow) };
                self.cfg.add_edge(src, dst, edge);
                dst
            }
            _ => {
                CFNode::Insn { address: prev_address.unwrap().offset() }
            }
        };

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
                        // callother may or may not be branching, but for the 
                        // purposes of lifting, we treat it as such.
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

            // connect the previous node to the current node
            let node = CFNode::Insn { address: address.offset() };
            let edge = CFEdge { src: prev_node, dst: node, flow: Some(FlowType::Fall) };
            self.cfg.add_edge(prev_node, node, edge);
            prev_node = node;

            self.cache.write().insert(address.offset(), Ok(insn));
        }
        // maybe return something here at some point?
    }

    fn _get_lift_result(&self, address: impl AsRef<Address>) -> LiftResult<'irb> {
        let address = address.as_ref();
        self.cache.read()
            .get(&address.offset())
            .ok_or(LiftError::AddressNotLifted(address.clone()))?
            .clone()
    }
}

impl<'irb> fmt::Debug for ProgramDB<'irb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arch = self.lang.translator().architecture();
        write!(f, "ProgramDB {{ {arch} }}")
    }
}