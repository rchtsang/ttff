//! jump.rs
//! 
//! tainted jump policy based on description from "All You Ever Wanted to 
//! Know..." by Schwartz, Avgerinos, and Brumley, 2010.
//! 
//! This policy is in fact a hybrid of the naive jump policy and naive 
//! address policy from the paper.
//! Here, violations are only triggered on a tainted jump, but we also
//! propagate the taint status of pointers to loaded/stored values, rather
//! than simply that of the value's source, in order to mitigate the
//! under-tainting problem.
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

    /// any tainted assignment to the program counter is a jump policy violation
    fn check_assign(
        &mut self,
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
        &mut self,
        _opcode: &Opcode,
        _cond: &(bool, Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
    }

    /// any attempt to branch to a target with a tainted source is a jump policy violation
    fn check_branch(
        &mut self,
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

    /// a loaded value is considered tainted if either the value at that 
    /// location was tainted, or if the pointer to the location was
    /// tainted
    fn propagate_load(
        &mut self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(Tag::new()
            .with_tainted_val(val.1.is_tainted())
            .with_tainted_loc(loc.1.is_tainted()))
    }
    
    /// a stored value is considered tainted if it came from a tainted
    /// source, or if the pointer used to store it was tainted.
    fn propagate_store(
        &mut self,
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