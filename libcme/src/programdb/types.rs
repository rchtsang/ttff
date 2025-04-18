//! types.rs
//! 
//! types for the program database
use std::hash::BuildHasherDefault;
use ahash;

pub use fugue_core::prelude::*;

use crate::types::*;

use petgraph::{
    Directed,
    graphmap::GraphMap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgramPoint { pub thread: EmuThread, pub address: Address }

/// a control flow graph node type
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum CFNode {
    ThreadEntry { id: EmuThread },
    FunctionEntry { start: u64 },
    FunctionExit { exit: u64 },
    Insn { address: u64 },
}

/// an instruction to instruction flow edge type
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct CFEdge {
    pub src: CFNode,
    pub dst: CFNode,
    pub flow: Option<FlowType>,
}

type AHashState = BuildHasherDefault<ahash::AHasher>;
pub type CFGraph = GraphMap<CFNode, CFEdge, Directed, AHashState>;