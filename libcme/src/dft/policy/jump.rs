//! control_flow.rs
//! 
//! defining a control flow integrity policy
//! 
//! a datum can be directly tainted or tainted by means of location offset.
//! any data derived from a tainted datum should be considered directly tainted.

use std::sync::Arc;

use thiserror::Error;

use fugue_core::language::Language;
use fugue_ir::{
    Address,
    VarnodeData,
    disassembly::Opcode,
};

use crate::dft::tag::Tag;
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
        &self,
        dst: &VarnodeData,
        tag: &Tag,
    ) -> Result<(), super::Error> {
        let t = self.lang.translator();
        if dst == t.program_counter() && tag.is_tainted() {
            Err(PolicyViolation::TaintedProgramCounter.into())
        } else {
            Ok(())
        }
    }

    fn check_branch(
        &self,
        opcode: &Opcode,
        tag: &Tag,
    ) -> Result<(), super::Error> {
        match opcode {
            Opcode::IBranch
            | Opcode::ICall
            | Opcode::Return if tag.is_tainted() => {
                Err(PolicyViolation::TaintedBranchTarget.into())
            }
            _ => { Ok(()) }
        }
    }

    fn check_write_mem(
        &self,
        _address: &Address,
        _tag: &Tag,
    ) -> Result<(), super::Error> {
        Ok(())
    }

    fn propogate_subpiece(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        src_tag: &Tag,
    ) -> Result<Tag, super::Error> {
        Ok(*src_tag)
    }
    
    fn propogate_int2(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs_tag: &Tag,
        rhs_tag: &Tag,
    ) -> Result<Tag, super::Error> {
        Ok(*lhs_tag | *rhs_tag)
    }
    
    fn propogate_int1(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs_tag: &Tag,
    ) -> Result<Tag, super::Error> {
        Ok(*rhs_tag)
    }
    
    fn propogate_bool2(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs_tag: &Tag,
        rhs_tag: &Tag,
    ) -> Result<Tag, super::Error> {
        Ok(*lhs_tag | *rhs_tag)
    }
    
    fn propogate_bool1(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs_tag: &Tag,
    ) -> Result<Tag, super::Error> {
        Ok(*rhs_tag)
    }

    fn propagate_load(
        &self,
        _dst: &VarnodeData,
        val_tag: &Tag,
        loc_tag: &Tag,
    ) -> Result<Tag, super::Error> {
        Ok(Tag::new()
            .with_tainted_val(loc_tag.is_tainted() || val_tag.is_tainted()))
    }
    
    fn propagate_store(
        &self,
        _dst: &VarnodeData,
        val_tag: &Tag,
        loc_tag: &Tag,
    ) -> Result<Tag, super::Error> {
        Ok(Tag::new()
            .with_tainted_val(val_tag.is_tainted())
            .with_tainted_loc(loc_tag.is_tainted()))
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