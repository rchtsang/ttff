//! programdb::plugin module
//! 
//! plugins for programdb
use std::fmt;

use anyhow;
use derive_more;
use thiserror::Error;

use crate::types::*;

use super::types::*;

mod dummy;
pub use dummy::DummyAnalysisPlugin;

/// allow arbitrary plugin error types
#[derive(Debug, derive_more::Display, Error)]
pub struct Error(pub(crate) anyhow::Error);


/// analysis plugin trait for programdb
#[allow(unused)]
pub trait AnalysisPlugin: fmt::Debug {
    /// callback invoked when any edge is encountered, before the edge
    /// is added to the cfg (it may not be)
    fn pre_edge_cb(
        &mut self,
        parent: u64,
        child: u64,
        flowtype: FlowType,
    ) -> Result<(), Error> { Ok(()) }

    /// callback invoked after a new block is lifted, after it has been
    /// added to the cfg.
    fn post_lift_block_cb<'z>(
        &mut self,
        block: &mut Block<'z>,
    ) -> () {  }
}


/// programdb plugin
/// 
/// a wrapper for the plugin(s) that will be provided to the
/// programdb upon instantiation
#[derive(Debug, Default)]
pub(crate) struct PDBPlugin {
    plugins: Vec<Box<dyn AnalysisPlugin>>,
}

impl PDBPlugin {
    pub fn add_plugin(&mut self, plugin: Box<dyn AnalysisPlugin>) {
        self.plugins.push(plugin)
    }
}

impl AnalysisPlugin for PDBPlugin {
    fn pre_edge_cb(
        &mut self,
        parent: u64,
        child: u64,
        flowtype: FlowType,
    ) -> Result<(), Error> {
        for plugin in self.plugins.iter_mut() {
            plugin.pre_edge_cb(parent, child, flowtype)?;
        }
        Ok(())
    }

    fn post_lift_block_cb<'z>(
        &mut self,
        block: &mut Block<'z>,
    ) {
        for plugin in self.plugins.iter_mut() {
            plugin.post_lift_block_cb(block);
        }
    }
}