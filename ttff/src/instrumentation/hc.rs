//! hc.rs
//! 
//! hit count eval plugin
use std::sync::Arc;
use parking_lot::RwLock;

use libcme::prelude::*;
use libcme::programdb::TranslationCache;

use super::CovMap;

/// hit counts eval plugin
#[derive(Debug, Clone)]
pub struct HcPlugin {
    covmap: CovMap,
}

impl HcPlugin {
    pub fn new(covmap: CovMap) -> Self {
        Self { covmap }
    }

    pub fn from_raw_parts(ptr: *mut [u8], size: usize) -> Self {
        Self { covmap: CovMap::new(ptr, size) }
    }
}

impl AnalysisPlugin for HcPlugin {
    #[instrument(skip_all)]
    fn pre_edge_cb(
        &mut self,
        parent: u64,
        child: u64,
        flowtype: FlowType,
    ) -> Result<(), programdb::plugin::Error> {
        let hash = (child ^ parent) & (self.covmap.size() as u64 - 1);
        let cnt = self.covmap[hash as usize];
        self.covmap[hash as usize] = cnt.wrapping_add(1);
        debug!(
            "edge hit: {flowtype:?} {{ {parent:#x} -> {child:#x} ; cnt = {}}}",
            self.covmap[hash as usize]);
        Ok(())
    }

    fn post_lift_block_cb<'z, 'irb>(
        &mut self,
        block: &mut Block<'z>,
        _cache: Arc<RwLock<TranslationCache<'irb>>>,
    ) -> () {
        info!(">> new block found: ({:#x}, {})",
            block.address(), block.size());
    }
}

// ulong hash = (pc ^ PREV_LOC) & (MAP_SIZE - 1);
// // lock (covMapLock){ 
// unsafe{
//     byte* ptr = (byte*)covMapPtr;
//     ptr[hash]++;
// }
// PREV_LOC = pc >> 1;