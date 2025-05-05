//! dummy.rs
//! 
//! a dummy analysis plugin

use crate::utils::*;
use super::*;


/// a dummy analysis plugin that counts callback invocations
#[derive(Debug, Default)]
pub struct DummyAnalysisPlugin {
    pub edges_encountered: usize,
    pub blocks_lifted: usize,
}

impl AnalysisPlugin for DummyAnalysisPlugin {
    #[instrument(skip_all)]
    fn pre_edge_cb(
        &mut self,
        _parent: u64,
        _child: u64,
        _flowtype: FlowType,
    ) -> Result<(), Error> {
        self.edges_encountered += 1;
        info!("edges encountered: {}", self.edges_encountered);
        Ok(())
    }

    #[instrument(skip_all)]
    fn post_lift_block_cb<'z, 'irb>(
        &mut self,
        _block: &mut Block<'z>,
        _cache: Arc<RwLock<TranslationCache<'irb>>>,
    ) -> () {
        self.blocks_lifted += 1;
        info!("blocks lifted: {}", self.blocks_lifted);
    }
}