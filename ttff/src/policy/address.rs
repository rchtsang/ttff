//! address.rs
//! 
//! tainted address policy based on "All You Ever Wanted to Know..." 
//! by Schwartz, Avgerinos, and Brumley, 2010
//! 
//! we add a condition that a violation should only be triggered if
//! the tainted address is outside of the current stack frame.
//! 
//! the frame information must be provided externally from a
//! callstack plugin that defines frame boundaries based on 
//! the sp at the time of calls and returns.
//! 
//! the frame information must be maintained locally and is updated 
//! based on what is provided.
use std::collections::VecDeque;
use std::sync::Arc;
use thiserror::Error;
use crossbeam::channel::{
    Sender,
    Receiver,
};

use fugue_bv::BitVec;
use fugue_core::language::Language;
use fugue_ir::{
    Address,
    VarnodeData,
    disassembly::Opcode,
};

use libcme::prelude::*;
use libcme::dft::{
    self,
    tag::Tag,
    policy::*,
};

/// control flow policy violations
#[derive(Clone, Error, Debug)]
pub enum PolicyViolation {
    #[error("read from memory using a tainted pointer")]
    TaintedRead,
    #[error("wrote to memory using a tainted address")]
    TaintedWrite,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FrameStart { pub pc: Address, pub sp: Address }

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FrameUpdate {
    Call(FrameStart),
    Return,
}

/// a control flow integrity policy to catch tainted PC writes
pub struct TaintedAddressPolicy {
    pub lang: Arc<Language>,
    pub call_channel: (Sender<FrameUpdate>, Receiver<FrameUpdate>),
    pub stack: VecDeque<FrameStart>,
}

impl TaintedAddressPolicy {
    pub fn new_with(
        lang: Arc<Language>,
        call_channel: (Sender<FrameUpdate>, Receiver<FrameUpdate>),
    ) -> Self {
        let stack = VecDeque::default();
        Self { lang, call_channel, stack }
    }

    pub fn update_stack(&mut self) {
        while let Ok(frame_update) = self.call_channel.1.try_recv() {
            match frame_update {
                FrameUpdate::Call(frame_start) => {
                    self.stack.push_back(frame_start);
                }
                FrameUpdate::Return => {
                    self.stack.pop_back();
                }
            }
        }
    }
}

impl TaintPolicy for TaintedAddressPolicy {

    fn check_assign(
        &mut self,
        _dst: &VarnodeData,
        _val: &(BitVec, Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
    }

    fn check_cond_branch(
        &mut self,
        _opcode: &Opcode,
        _cond: &(bool, Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
    }

    fn check_branch(
        &mut self,
        _opcode: &Opcode,
        _target: &(Address, Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
    }

    fn check_write_mem(
        &mut self,
        _address: &Address,
        _val: (&BitVec, &Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
    }

    /// preserve the tag of the source
    fn propagate_subpiece(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        src: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(src.1)
    }
    
    /// the result tag of any binary operation is a bitwise or of its
    /// parameters' tags
    fn propagate_int2(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(lhs.1 | rhs.1)
    }
    
    /// the result tag of any unary operation is the same as its
    /// parameter's tag
    fn propagate_int1(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(rhs.1)
    }
    
    /// the result tag of any binary operation is a bitwise or of its
    /// parameters' tags
    fn propagate_bool2(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(lhs.1 | rhs.1)
    }
    
    /// the result tag of any unary operation is the same as its
    /// parameter's tag
    fn propagate_bool1(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(rhs.1)
    }

    /// in the tainted address policy, a loaded value is tainted if the value at
    /// the read location was tainted, while a load using a tainted address
    /// will trigger a policy violation
    fn propagate_load<'a>(
        &mut self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
        ctx: &dft::Context<'a>,
    ) -> Result<Tag, policy::Error> {
        if loc.1.is_tainted() {
            self.update_stack();
            let sp = ctx.backend().read_sp().expect("failed to read sp");
            if let Some(frame_start) = self.stack.back() {
                if loc.0 > frame_start.sp || loc.0 < sp {
                    return Err(policy::Error::from(PolicyViolation::TaintedRead))
                }
            }
        }
        Ok(Tag::new().with_tainted_val(val.1.is_tainted()))
    }
    
    /// in the tainted address policy, a stored value is tainted if  
    /// the source of the value was tainted, while a store using a tainted
    /// address will trigger a policy violation
    fn propagate_store<'a>(
        &mut self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
        ctx: &dft::Context<'a>,
    ) -> Result<Tag, policy::Error> {
        if loc.1.is_tainted() {
            self.update_stack();
            let sp = ctx.backend().read_sp().expect("failed to read sp");
            if let Some(frame_start) = self.stack.back() {
                if loc.0 > frame_start.sp || loc.0 < sp {
                    return Err(policy::Error::from(PolicyViolation::TaintedWrite))
                }
            }
        }
        Ok(Tag::new().with_tainted_val(val.1.is_tainted()))
    }
}

impl std::fmt::Debug for TaintedAddressPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arch = self.lang.translator().architecture();
        write!(f, "TaintedAddressPolicy {{ {arch} }}")
    }
}

impl From<PolicyViolation> for policy::Error {
    fn from(value: PolicyViolation) -> Self {
        policy::Error::from(anyhow::Error::from(value))
    }
}