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
use derive_more;
use thiserror::Error;

use fugue_core::prelude::*;
use fugue_core::ir::Location;

use crate::types::*;
use crate::dft::tag::Tag;

/// arbitrary hook errors
#[derive(Debug, derive_more::Display, Error)]
pub struct Error(pub(crate) anyhow::Error);


pub trait Plugin {
    pub fn pre_insn_cb<'irb, 'backend>(
        &'mut self,
        loc: &Location,
        insn: &Insn<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>;

    pub fn pre_pcode_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        pcode: &PCodeData<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>;

    pub fn pre_mem_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        mem_address: &Address,
        mem_size: usize,
        access_type: Permission,
        value: (BitVec, Tag),
        context: &mut Context<'backend>,
    ) -> Reuslt<(), Error>;
}
