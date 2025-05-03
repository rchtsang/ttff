//! address.rs
//! 
//! tainted address policy as described in "All You Ever Wanted to Know..." 
//! by Schwartz, Avgerinos, and Brumley, 2010
//! 
//! this is the naive, overtainting version as originally presented,
//! which will trigger a crash on every tainted memory access.
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
    #[error("read from memory using a tainted pointer")]
    TaintedRead,
    #[error("wrote to memory using a tainted address")]
    TaintedWrite,
}

/// a control flow integrity policy to catch tainted PC writes
pub struct TaintedAddressPolicy {
    pub lang: Arc<Language>,
}

impl TaintedAddressPolicy {
    pub fn new_with(lang: Arc<Language>) -> Self {
        Self { lang }
    }
}

impl TaintPolicy for TaintedAddressPolicy {

    fn check_assign(
        &self,
        _dst: &VarnodeData,
        _val: &(BitVec, Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
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
        _opcode: &Opcode,
        _target: &(Address, Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
    }

    fn check_write_mem(
        &self,
        _address: &Address,
        _val: (&BitVec, &Tag),
    ) -> Result<(), policy::Error> {
        Ok(())
    }

    /// preserve the tag of the source
    fn propagate_subpiece(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        src: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(src.1)
    }
    
    /// the result tag of any binary operation is a bitwise or of its
    /// parameters' tags
    fn propagate_int2(
        &self,
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
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(rhs.1)
    }
    
    /// the result tag of any binary operation is a bitwise or of its
    /// parameters' tags
    fn propagate_bool2(
        &self,
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
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        Ok(rhs.1)
    }

    /// in the tainted address policy, a loaded value is tainted if the value at
    /// the read location was tainted, while a load using a tainted address
    /// will trigger a policy violation
    fn propagate_load(
        &self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
    ) -> Result<Tag, policy::Error> {
        if loc.1.is_tainted() {
            Err(policy::Error::from(PolicyViolation::TaintedRead))
        } else {
            Ok(Tag::new().with_tainted_val(val.1.is_tainted()))
        }
    }
    
    /// in the tainted address policy, a stored value is tainted if  
    /// the source of the value was tainted, while a store using a tainted
    /// address will trigger a policy violation
    fn propagate_store(
        &self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
    ) -> Result<Tag, policy::Error> {
        if loc.1.is_tainted() {
            Err(policy::Error::from(PolicyViolation::TaintedWrite))
        } else {
            Ok(Tag::new().with_tainted_val(val.1.is_tainted()))
        }
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