//! types.rs
//! 
//! smaller types and structs used in programdb module
use std::ops::Range;
use nohash::IntMap;
pub use bumpalo::{
    Bump as BumpArena,
    collections::Vec as BumpVec,
};

use crate::types::*;

pub type TranslationCache<'irb> = IntMap<u64, LiftResult<'irb>>;


/// translation block
#[derive(Debug, Clone)]
pub struct Block<'arena> {
    range: Range<u64>,
    insns: BumpVec<'arena, u64>,
    successors: BumpVec<'arena, (FlowType, u64)>,
}

impl<'arena> Block<'arena> {

    pub fn new_in(
        bump: &'arena BumpArena,
        range: Range<u64>,
        insns: impl IntoIterator<Item=u64>,
        successors: impl IntoIterator<Item=(FlowType, u64)>,
    ) -> Self {
        let insns = BumpVec::from_iter_in(insns, bump);
        let successors = BumpVec::from_iter_in(successors, bump);
        Self { range, insns, successors }
    }

    pub fn address(&self) -> u64 {
        self.range.start
    }

    pub fn size(&self) -> usize {
        (self.range.end - self.range.start) as usize
    }

    pub fn range(&self) -> &Range<u64> {
        &self.range
    }

    pub fn insns(&self) -> &[u64] {
        &self.insns[..]
    }

    pub fn successors(&self) -> &[(FlowType, u64)] {
        &self.successors[..]
    }

    pub fn add_successor(&mut self, child: u64, flowtype: FlowType) {
        self.successors.push((flowtype, child))
    }
}


