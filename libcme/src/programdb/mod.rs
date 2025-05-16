//! programdb module
//! 
use std::fmt;
use std::sync::Arc;
use std::io::{BufWriter, Write};

use thiserror::Error;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json;
use petgraph::Direction;

// use fugue_core::prelude::*;
use fugue_ir::disassembly::{ Opcode, IRBuilderArena };

use crate::types::*;
use crate::utils::*;
use crate::backend::Backend;

pub mod plugin;
pub use plugin::*;
mod cfg;
pub use cfg::*;
mod types;
pub use types::*;
pub mod platform;
pub use platform::{
    MmioRegion,
    MemRegion,
    Platform,
};
pub mod program;
pub use program::Program;

/// programdb errors
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CFG(#[from] cfg::Error),
    #[error(transparent)]
    Platform(#[from] platform::Error),
    #[error("plugin error: {0}")]
    Plugin(anyhow::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}


/// A ProgramDB should serve as an intermediary between the evaluator
/// and the context for the purposes of fetching and lifting instructions.
/// All lifted instructions should be stored in the ProgramDB, so that 
/// separate analysis can be conducted without the evaluator context,
/// independently of whether execution has taint tracking enabled or not.
pub struct ProgramDB<'irb> {
    pub(crate) lang: Language,
    pub(crate) platform: Platform,
    pub(crate) program: Program<'irb>,
    pub(crate) cache: Arc<RwLock<TranslationCache<'irb>>>,
    pub(crate) arena: &'irb IRBuilderArena,
    pub(crate) cfg: CFGraph<'irb>,
    plugin: PDBPlugin,
}


impl<'irb> ProgramDB<'irb> {

    pub fn new_with(
        builder: &LanguageBuilder,
        program: Program<'irb>,
        platform: Platform,
        arena: &'irb IRBuilderArena,
    ) -> Self {
        let maybe_lang = platform.lang(builder);
        let lang = match maybe_lang {
            Ok(lang) => { lang }
            Err(err) => { panic!("{err}") }
        };

        let cache = Arc::new(RwLock::new(TranslationCache::default()));
        let cfg = CFGraph::new_with(arena.inner());
        let plugin = PDBPlugin::default();

        Self { lang, platform, program, cache, arena, cfg, plugin }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn AnalysisPlugin>) {
        self.plugin.add_plugin(plugin)
    }

    pub fn fetch(&mut self, address: Address, backend: &mut impl Backend) -> LiftResult<'irb> {
        let address: Address = (address.offset() & !1).into();

        if !self.cache.read().contains_key(&address.offset()) {
            self._lift_block(address, backend);
        }

        self.cache.read()
            .get(&address.offset())
            .ok_or(LiftError::AddressNotLifted(address.into()))?
            .clone()
    }

    pub fn add_edge(&mut self, parent: Address, child: Address, flowtype: FlowType) -> Result<(), Error> {
        let (parent, child) = (parent.into(), child.into());
        self.plugin.pre_edge_cb(parent, child, flowtype)?;
        self.cfg.add_edge(parent, child, flowtype)
            .map_err(Error::from)
    }

    pub fn is_block_end(&self, address: impl Into<u64>) -> bool {
        let address = address.into();
        let Some(block) = self.cfg.get_block(address) else {
            // address not in a block
            return false;
        };
        // unwrap because there shouldn't be an empty block if it was found
        let last_insn = block.insns().last().unwrap();
        *last_insn == address
    }

    pub fn platform(&self) -> &Platform {
        &self.platform
    }

    pub fn program(&self) -> &Program<'irb> {
        &self.program
    }

    pub fn lang(&self) -> &Language {
        &self.lang
    }

    pub fn backend(&self, builder: &LanguageBuilder) -> Result<impl Backend, Error> {
        self.platform.backend(builder).map_err(Error::from)
    }
}

impl<'irb> ProgramDB<'irb> {
    #[instrument(skip_all)]
    fn _lift_block(&mut self,
        address: impl Into<Address> + fmt::Debug,
        backend: &mut impl Backend,
    ) {
        let base = address.into();
        let mut offset = 0usize;

        let mut insns = vec![];
        
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

            debug!("{}", backend.fmt_pcode(pcode));

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

        let range = base.offset()..(base.offset() + offset as u64);
        let insns = insns.into_iter().map(|addr| addr.offset());
        let block = Block::new_in(self.arena.inner(), range, insns, vec![]);

        if let Err(err) = self.cfg.add_block(block, None) {
            panic!("unexpected error: {err:?}");
        }

        self.plugin.post_lift_block_cb(
            self.cfg.get_block_mut(base.offset()).unwrap(),
            self.cache.clone(),
        );
        // maybe return something here at some point?
    }
}


impl<'irb> ProgramDB<'irb> {
    /// dump the cfg into the writer
    pub fn dump_cfg(
        &self,
        mut writer: BufWriter<impl Write>,
    ) -> Result<(), Error> {
        #[derive(Serialize, Deserialize)]
        struct SimpleBBlock {
            pub address: u32,
            pub size: usize,
            pub insn_addrs: Vec<u32>,
            pub predecessors: Vec<u32>,
            pub successors: Vec<u32>,
        }

        #[derive(Serialize, Deserialize)]
        struct SimpleCFG {
            pub nodes: Vec<SimpleBBlock>,
        }

        let mut blocks: Vec<SimpleBBlock> = vec![];
        for (&address, block) in self.cfg.blocks() {
            let address = address as u32;
            let size = (block.range().end - block.range().start) as usize;
            let insn_addrs: Vec<u32> = block.insns()
                .iter()
                .map(|addr| *addr as u32)
                .collect();
            let sblock = SimpleBBlock {
                address,
                size,
                insn_addrs,
                predecessors: vec![],
                successors: vec![],
            };
            blocks.push(sblock);
        }
        for block in blocks.iter_mut() {
            block.predecessors = self.cfg.graph()
                .edges_directed(block.address as u64, Direction::Incoming)
                .map(|(from, _to, _flowtype)| { from as u32 })
                .collect();
            block.successors = self.cfg.graph()
                .edges_directed(block.address as u64, Direction::Outgoing)
                .map(|(_from, to, _flowtype)| { to as u32 })
                .collect();
        }
        let cfg = SimpleCFG { nodes: blocks };
        let cfg_str = serde_json::to_string(&cfg)?;
        writer.write(cfg_str.as_bytes())?;
        Ok(())
    }
}


impl<'irb> fmt::Debug for ProgramDB<'irb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arch = self.lang.translator().architecture();
        write!(f, "ProgramDB {{ {arch} }}")
    }
}

impl From<plugin::Error> for Error {
    fn from(err: plugin::Error) -> Self {
        Self::Plugin(err.0)
    }
}