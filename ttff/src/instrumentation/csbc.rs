//! csbc.rs
//! 
//! context-sensitive branch coverage plugin
use std::collections::VecDeque;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use nohash::IntMap;

use libcme::prelude::*;

use super::CovMap;

/// a context-sensitive branch coverage plugin
/// based vaguely on Angora.
#[derive(Debug, Clone)]
pub struct CsbcPlugin {
    rng: Pcg64Mcg,
    callstack: VecDeque<u64>,
    callees: IntMap<u64, u64>,
    context: u64,
    covmap: CovMap,
}

impl CsbcPlugin {
    pub fn new(covmap: CovMap) -> Self {
        let seed = 0; // we seed with 0 for reproducibility
        let rng = Pcg64Mcg::new(seed);
        let callees = IntMap::default();
        let callstack = VecDeque::default();
        Self { rng, callstack, callees, context: 0, covmap }
    }

    pub fn from_raw_parts(ptr: *mut [u8], size: usize) -> Self {
        let seed = 0; // we seed with 0 for reproducibility
        let rng = Pcg64Mcg::new(seed);
        let callees = IntMap::default();
        let callstack = VecDeque::default();
        Self { rng, callstack, callees, context: 0, covmap: CovMap::new(ptr, size) }
    }
}

impl AnalysisPlugin for CsbcPlugin {
    #[instrument(skip_all)]
    fn pre_edge_cb(
        &mut self,
        parent: u64,
        child: u64,
        flowtype: FlowType,
    ) -> Result<(), programdb::plugin::Error> {
        let hash = (child ^ parent ^ self.context) & (self.covmap.size() as u64 - 1);
        self.covmap[hash as usize] += 1;
        debug!(
            "edge hit: {flowtype:?} {{ {parent:#x} -> {child:#x} ; ctx={:#x} ; cnt={} }}",
            self.context,
            self.covmap[hash as usize]);

        // update context after the hash so that the hash is associated with the
        // calling/returning context instead of the target context.
        match flowtype {
            FlowType::Call
            | FlowType::ICall => {
                let id = self.callees.entry(child)
                    .or_insert(self.rng.random());
                self.callstack.push_back(child);
                self.context ^= *id;
            }
            FlowType::Return => {
                let callee = self.callstack.pop_back()
                    .ok_or_else(|| {
                        let msg = "returned with no callstack";
                        let err = anyhow::Error::msg(msg);
                        programdb::plugin::Error(err)
                    })?;
                let id = self.callees.get(&callee)
                    .ok_or_else(|| {
                        let msg = "callee has no generated id";
                        let err = anyhow::Error::msg(msg);
                        programdb::plugin::Error(err)
                    })?;
                self.context ^= id;
            }
            _ => {  }
        }
        Ok(())
    }
}

// ulong hash = (pc ^ PREV_LOC) & (MAP_SIZE - 1);
// // lock (covMapLock){ 
// unsafe{
//     byte* ptr = (byte*)covMapPtr;
//     ptr[hash]++;
// }
// PREV_LOC = pc >> 1;