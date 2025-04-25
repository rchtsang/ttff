//! plugin module
//! 
//! this module defines the plugin trait that a dft plugin must implement.
//! 
//! the plugin trait defines a set of callback functions that the evaluator
//! invokes during execution that can be used for analysis and modification of
//! the emulation state.
//! 
//! plugins are given mutable references to the context, but cannot instrument
//! actual instructions due to current limitations of fugue (pcode currently
//! cannot be modified or cloned)
use std::fmt;
use derive_more;
use thiserror::Error;

use fugue_core::prelude::*;
use fugue_core::ir::Location;
use fugue_ir::disassembly::{PCodeData, VarnodeData};

use crate::types::*;
use crate::dft::Context;
use crate::dft::tag::Tag;

mod dummy;
pub use dummy::DummyEvalPlugin;

/// allow arbitrary plugin error types
#[derive(Debug, derive_more::Display, Error)]
pub struct Error(pub(crate) anyhow::Error);


/// plugin trait for evaluator
#[allow(unused)]
pub trait EvalPlugin: fmt::Debug {

    fn pre_insn_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        insn: &Insn<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> { Ok(()) }
    
    fn post_insn_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        insn: &Insn<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> { Ok(()) }

    fn pre_pcode_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        pcode: &PCodeData<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> { Ok(()) }

    fn post_pcode_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        pcode: &PCodeData<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> { Ok(()) }

    fn mem_access_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        mem_address: &Address,
        mem_size: usize,
        access_type: Permission,
        value: &mut (BitVec, Tag),
        context: &mut Context<'backend>,
    ) -> Result<(), Error> { Ok(()) }

    fn pre_userop_cb<'backend>(
        &mut self,
        loc: &Location,
        index: usize,
        inputs: &[VarnodeData],
        output: Option<&VarnodeData>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> { Ok(()) }

    fn post_userop_cb<'backend>(
        &mut self,
        loc: &Location,
        index: usize,
        inputs: &[VarnodeData],
        output: Option<&VarnodeData>,
        context: &mut Context<'backend>,
        result: &Option<Location>,
    ) -> Result<(), Error> { Ok(()) }
}


/// evaluator plugin
/// 
/// a wrapper for the plugin(s) that will be provided to the
/// evaluator upon instantiation
#[derive(Debug)]
pub(crate) struct EvaluatorPlugin {
    plugins: Vec<Box<dyn EvalPlugin>>,
}

impl EvaluatorPlugin {
    pub(crate) fn new_with(plugins: Vec<Box<dyn EvalPlugin>>) -> Self {
        Self { plugins }
    }

    pub(crate) fn add_plugin(&mut self, plugin: Box<dyn EvalPlugin>) {
        self.plugins.push(plugin)
    }
}

impl Default for EvaluatorPlugin {
    fn default() -> Self {
        Self::new_with(vec![])
    }
}

impl EvalPlugin for EvaluatorPlugin {

    fn pre_insn_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        insn: &Insn<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        for plugin in self.plugins.iter_mut() {
            plugin.as_mut().pre_insn_cb(loc, insn, context)?;
        }
        Ok(())
    }

    fn post_insn_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        insn: &Insn<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        for plugin in self.plugins.iter_mut() {
            plugin.as_mut().post_insn_cb(loc, insn, context)?;
        }
        Ok(())
    }

    fn pre_pcode_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        pcode: &PCodeData<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        for plugin in self.plugins.iter_mut() {
            plugin.as_mut().pre_pcode_cb(loc, pcode, context)?;
        }
        Ok(())
    }

    fn post_pcode_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        pcode: &PCodeData<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        for plugin in self.plugins.iter_mut() {
            plugin.as_mut().post_pcode_cb(loc, pcode, context)?;
        }
        Ok(())
    }

    fn mem_access_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        mem_address: &Address,
        mem_size: usize,
        access_type: Permission,
        value: &mut (BitVec, Tag),
        context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        for plugin in self.plugins.iter_mut() {
            plugin.as_mut().mem_access_cb(loc, mem_address, mem_size, access_type, value, context)?;
        }
        Ok(())
    }
}