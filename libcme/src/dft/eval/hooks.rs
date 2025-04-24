//! hooks.rs
//! 
//! hooks/callbacks that can be registered on the evaluator
use std::fmt;
use std::ops::Range;

use thiserror::Error;
use derive_more;
use anyhow;

use fugue_core::prelude::*;
use fugue_ir::disassembly::PCodeData;

use crate::types::*;
use crate::dft::tag::Tag;
use super::Context;

/// arbitrary hook errors
#[derive(Debug, derive_more::Display, Error)]
pub struct Error(pub(crate) anyhow::Error);


/// hook types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HookType {
    PreInsn,
    PostInsn,
    PrePCode,
    PostPCode,
    PreMemAccess,
    PostMemAccess,
}

#[derive(Debug)]
pub enum HookState {
    PCode { state: Box<dyn PCodeHookState> },
    Insn { state: Box<dyn InsnHookState> },
    MemAccess { state: Box<dyn MemAccessHookState> },
}

/// hook
#[derive(Debug)]
pub struct Hook {
    pub typ: HookType,
    pub addresses: Option<Range<Address>>,
    pub state: HookState,
}

pub trait PCodeHookState: fmt::Debug {
    fn callback<'backend, 'irb>(
        &mut self,
        pc_address: &Address,
        pcode: &PCodeData<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>;
}

pub trait InsnHookState: fmt::Debug {
    fn callback<'backend, 'irb>(
        &mut self,
        pc_address: &Address,
        insn: &Insn<'irb>,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>;
}

pub trait MemAccessHookState: fmt::Debug {
    fn callback<'backend, 'irb>(
        &mut self,
        pc_address: &Address,
        mem_address: &Address,
        mem_size: usize,
        access_type: Permission,
        value: (BitVec, Tag),
        context: &mut Context<'backend>,
    ) -> Result<(), Error>;
}