//! control_flow.rs
//! 
//! defining a control flow integrity policy
//! 
//! a datum can be directly tainted or tainted by means of location offset.
//! any data derived from a tainted datum should be considered directly tainted.

use std::sync::Arc;

use thiserror::Error;

use fugue_bv::BitVec;
use fugue_core::language::Language;
use fugue_ir::{
    Address,
    VarnodeData,
    disassembly::Opcode,
};

use crate::dft::{self, tag::Tag};
use super::TaintPolicy;

/// control flow policy violations
#[derive(Clone, Error, Debug)]
pub enum PolicyViolation {
    #[error("assigned a tainted value to the program counter")]
    TaintedProgramCounter,
    #[error("branched to a tainted address")]
    TaintedBranchTarget,
}

/// a control flow integrity policy to catch tainted PC writes
pub struct TaintedJumpPolicy {
    pub lang: Arc<Language>,
}

impl TaintedJumpPolicy {
    pub fn new_with(lang: Arc<Language>) -> Self {
        Self { lang }
    }
}

impl TaintPolicy for TaintedJumpPolicy {

    fn check_assign(
        &mut self,
        dst: &VarnodeData,
        val: &(BitVec, Tag),
    ) -> Result<(), super::Error> {
        let t = self.lang.translator();
        if dst == t.program_counter() && val.1.is_tainted() {
            Err(PolicyViolation::TaintedProgramCounter.into())
        } else {
            Ok(())
        }
    }

    fn check_cond_branch(
        &mut self,
        _opcode: &Opcode,
        _cond: &(bool, Tag),
    ) -> Result<(), super::Error> {
        Ok(())
    }

    fn check_branch(
        &mut self,
        opcode: &Opcode,
        target: &(Address, Tag),
    ) -> Result<(), super::Error> {
        match opcode {
            Opcode::IBranch
            | Opcode::ICall
            | Opcode::Return if target.1.is_tainted() => {
                Err(PolicyViolation::TaintedBranchTarget.into())
            }
            _ => { Ok(()) }
        }
    }

    fn check_write_mem(
        &mut self,
        _address: &Address,
        _val: (&BitVec, &Tag),
    ) -> Result<(), super::Error> {
        Ok(())
    }

    fn propagate_subpiece(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        src: &(BitVec, Tag),
    ) -> Result<Tag, super::Error> {
        Ok(src.1)
    }
    
    fn propagate_int2(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, super::Error> {
        Ok(lhs.1 | rhs.1)
    }
    
    fn propagate_int1(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, super::Error> {
        Ok(rhs.1)
    }
    
    fn propagate_bool2(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, super::Error> {
        Ok(lhs.1 | rhs.1)
    }
    
    fn propagate_bool1(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, super::Error> {
        Ok(rhs.1)
    }

    fn propagate_load<'a>(
        &mut self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
        _ctx: &dft::Context<'a>,
    ) -> Result<Tag, super::Error> {
        Ok(Tag::new()
            .with_tainted_val(loc.1.is_tainted() || val.1.is_tainted()))
    }
    
    fn propagate_store<'a>(
        &mut self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
        _ctx: &dft::Context<'a>,
    ) -> Result<Tag, super::Error> {
        Ok(Tag::new()
            .with_tainted_val(val.1.is_tainted())
            .with_tainted_loc(loc.1.is_tainted()))
    }
}

impl std::fmt::Debug for TaintedJumpPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arch = self.lang.translator().architecture();
        write!(f, "TaintedJumpPolicy {{ {arch} }}")
    }
}

impl From<PolicyViolation> for super::Error {
    fn from(value: PolicyViolation) -> Self {
        Self(anyhow::Error::from(value))
    }
}