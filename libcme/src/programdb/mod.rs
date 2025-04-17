//! programdb module
//! 
use std::fmt;
use std::sync::Arc;

use thiserror::Error;
use parking_lot::RwLock;
use nohash::IntMap;

use fugue_core::prelude::*;
use fugue_ir::disassembly::IRBuilderArena;

use crate::types::*;

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
    lang: Arc<Language>,
    cache: Arc<RwLock<TranslationCache<'irb>>>,
    arena: &'irb IRBuilderArena,
}


impl<'irb> ProgramDB<'irb> {

    pub fn new_with(lang: Arc<Language>, arena: &'irb IRBuilderArena) -> Self {
        let cache = Arc::new(RwLock::new(TranslationCache::default()));

        Self { lang, cache, arena }
    }

}

impl<'irb> ProgramDB<'irb> {

}

impl<'irb> fmt::Debug for ProgramDB<'irb> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arch = self.lang.translator().architecture();
        write!(f, "ProgramDB {{ {arch} }}")
    }
}