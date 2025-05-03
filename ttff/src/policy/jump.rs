//! jump.rs
//! 
//! tainted jump policy as described in 
//! "All You Ever Wanted to Know..." by
//! Schwartz, Avgerinos, and Brumley, 2010
use std::sync::Arc;
use thiserror::Error;

use fugue_bv::BitVec;
use fugue_core::language::Language;
use fugue_ir::{
    Address,
    VarnodeData,
    disassembly::Opcode,
};

use libcme::prelude::*;
use libcme::dft::{
    tag::Tag,
    policy::*,
};

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
        val: &(BitVec, Tag),
    ) -> Result<(), policy::Error> {
        let t = self.lang.translator();
        if dst == t.program_counter() && val.1.is_tainted() {
            Err(PolicyViolation::TaintedProgramCounter.into())
        } else {
            Ok(())
        }
    }

    fn check_cond_branch(
        &self,
        _opcode: &Opcode,
        _cond: &(bool, Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
    }

    fn check_branch(
        &self,
        opcode: &Opcode,
        target: &(Address, Tag),
    ) -> Result<(), policy::Error> {
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
        &self,
        _address: &Address,
        _val: (&BitVec, &Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
    }

    fn propagate_subpiece(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        src: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(src.1)
    }
    
    fn propagate_int2(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(lhs.1 | rhs.1)
    }
    
    fn propagate_int1(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(rhs.1)
    }
    
    fn propagate_bool2(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(lhs.1 | rhs.1)
    }
    
    fn propagate_bool1(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(rhs.1)
    }

    fn propagate_load(
        &self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(Tag::new()
            .with_tainted_val(loc.1.is_tainted() || val.1.is_tainted()))
    }
    
    fn propagate_store(
        &self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
    ) -> Result<Tag, policy::Error> {
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

impl From<PolicyViolation> for policy::Error {
    fn from(value: PolicyViolation) -> Self {
        policy::Error::from(anyhow::Error::from(value))
    }
}