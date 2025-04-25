//! cfg.rs
//! 
//! control flow graph implementation
//! 
//! the CFGraph doesn't store actual instructions, just the addresses.
//! instructions should be fetched from the programdb's cache.
use std::fmt;
use std::ops::Range;
use std::hash::BuildHasherDefault;
// use itertools::Itertools;
use thiserror::Error;
use ahash;
use nohash::IntMap;
use iset::IntervalMap;
use petgraph::{
    Directed,
    graphmap::GraphMap,
};

pub use fugue_core::prelude::*;

use crate::types::*;
use crate::utils::*;

use super::types::*;

type AHashState = BuildHasherDefault<ahash::AHasher>;


#[derive(Error, Debug)]
pub enum Error {
    #[error("block @ {0:#x} already translated")]
    BlockAlreadyExists(u64),
    #[error("block @ {0:#x} does not exist")]
    BlockDoesNotExist(u64),
}

/// control flow graph
/// 
/// by default, is not normalized. must be normalized before analysis
pub struct CFGraph<'arena> {
    bump: &'arena BumpArena,
    blocks: IntMap<u64, Block<'arena>>,
    blkmap: IntervalMap<u64, u64>,
    graph: GraphMap<u64, FlowType, Directed, AHashState>,
}

impl<'arena> CFGraph<'arena> {
    
    pub fn new_with(bump: &'arena BumpArena) -> Self {
        let blocks = IntMap::default();
        let blkmap = IntervalMap::default();
        let graph = GraphMap::default();
        Self { bump, blocks, blkmap, graph }
    }

    pub fn new_block(
        &mut self,
        range: Range<u64>,
        insns: impl IntoIterator<Item=u64> + fmt::Debug,
        successors: impl IntoIterator<Item=(FlowType, u64)> + fmt::Debug,
        parent: Option<(FlowType, u64)>,
    ) -> Result<(), Error> {
        let block = Block::new_in(self.bump, range, insns, successors);
        self.add_block(block, parent)
    }

    #[instrument]
    pub fn add_block(
        &mut self,
        block: Block<'arena>,
        parent: Option<(FlowType, u64)>,
    ) -> Result<(), Error> {
        let address = block.address();
        if self.blocks.contains_key(&address) {
            return Err(Error::BlockAlreadyExists(block.address()));
        }
        
        for (flowtype, target) in block.successors() {
            if let Some(edge) = self.graph.add_edge(address, *target, *flowtype) {
                warn!("edge already exists: {edge:?}({address:#x} -> {target:#x})");
            }
        }

        if let Some((flowtype, parent)) = parent {
            if let Some(edge) = self.graph.add_edge(parent, address, flowtype) {
                warn!("edge already exists: {edge:?}({parent:#x} -> {address:#x})");
            }
        }

        self.blkmap.insert(block.range().clone(), address);
        self.blocks.insert(address, block);

        Ok(())
    }

    #[instrument]
    pub fn add_edge(
        &mut self,
        parent: u64,
        child: u64,
        flowtype: FlowType,
    ) -> Result<(), Error> {
        let Some((_, parent_base)) = self.blkmap.overlap(parent).next() else {
            return Err(Error::BlockDoesNotExist(parent))
        };
        assert_eq!(
            *self.blocks.get(parent_base).unwrap().insns().last().unwrap(),
            parent,
            "parent must be last instruction in its block",
        );
        let child_base = self.blkmap.overlap(child).next()
            .map(|(_, child_base)| *child_base)
            .unwrap_or(child);
        if let Some(edge) = self.graph.add_edge(*parent_base, child_base, flowtype) {
            warn!("edge already exists: {edge:?}({parent_base:#x} -> {child:#x})");
        } else {
            self.blocks.entry(*parent_base)
                .and_modify(|block| block.add_successor(child_base, flowtype));
        }

        Ok(())
    }

    pub fn get_block(&self, address: impl Into<u64>) -> Option<&Block<'arena>> {
        let address = address.into();
        let (_, blk_address) = self.blkmap.overlap(address).next()?;
        self.blocks.get(blk_address)
    }

    pub fn get_block_mut(&mut self, address: impl Into<u64>) -> Option<&mut Block<'arena>> {
        let address = address.into();
        let (_, blk_address) = self.blkmap.overlap(address).next()?;
        self.blocks.get_mut(blk_address)
    }
}

impl<'arena> fmt::Debug for CFGraph<'arena> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CFGraph [ {} blocks ; {} edges ]",
            self.blocks.len(),
            self.graph.edge_count(),
        )
    }
}