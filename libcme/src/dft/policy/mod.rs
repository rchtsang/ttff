//! policy.rs
//! 
//! defining taint policies
use thiserror::Error;
use fugue_bv::BitVec;
use fugue_core::prelude::*;
use fugue_ir::disassembly::{
    VarnodeData,
    Opcode,
};
use super::tag::{self, Tag};

pub mod jump;
pub use jump::{
    TaintedJumpPolicy,
    PolicyViolation as TaintedJumpPolicyViolation,
};

#[derive(Debug, derive_more::Display, Error)]
pub struct Error(pub(crate) anyhow::Error);

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

#[derive(Debug)]
pub struct EvalPolicy<'policy> {
    pub inner: Box<dyn TaintPolicy + 'policy>,
}

pub trait TaintPolicy: std::fmt::Debug {
    /// check for policy violations on varnode assignment
    fn check_assign(
        &mut self,
        dst: &VarnodeData,
        val: &(BitVec, Tag),
    ) -> Result<(), Error>;

    /// check for policy violations on memory write
    fn check_write_mem(
        &mut self,
        address: &Address,
        val: (&BitVec, &Tag),
    ) -> Result<(), Error>;
    
    /// check for policy violations on conditional branches
    fn check_cond_branch(
        &mut self,
        opcode: &Opcode,
        cond: &(bool, Tag),
    ) -> Result<(), Error>;

    /// check for policy violations on target branches
    fn check_branch(
        &mut self,
        opcode: &Opcode,
        target: &(Address, Tag),
    ) -> Result<(), Error>;

    /// check for policy violations during subpiece operations.
    /// return error if violation detected, otherwise propagate taint
    fn propagate_subpiece(
        &mut self,
        opcode: &Opcode,
        dst: &VarnodeData,
        src: &(BitVec, Tag),
    ) -> Result<Tag, Error>;
    
    /// check for policy violations during operations involving 2 integers.
    /// return error if violation detected, otherwise propagate taint
    fn propagate_int2(
        &mut self,
        opcode: &Opcode,
        dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error>;
    
    /// check for policy violations during operations involving 2 integer.
    /// return error if violation detected, otherwise propagate taint
    fn propagate_int1(
        &mut self,
        opcode: &Opcode,
        dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error>;
    
    /// check for policy violations during operations involving 2 booleans.
    /// return error if violation detected, otherwise propagate taint
    fn propagate_bool2(
        &mut self,
        opcode: &Opcode,
        dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error>;
    
    /// check for policy violations during operations involving 1 boolean.
    /// return error if violation detected, otherwise propagate taint
    fn propagate_bool1(
        &mut self,
        opcode: &Opcode,
        dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error>;

    /// check for policy violations during load operations
    /// return error if violation detected, otherwise propagate taint
    fn propagate_load(
        &mut self,
        dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
    ) -> Result<Tag, Error>;

    /// check for policy violations during store operations
    /// return error if violation detected, otherwise propagate taint
    fn propagate_store(
        &mut self,
        dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
    ) -> Result<Tag, Error>;
}

/// a dummy policy that has no violations and no propagation
/// should be identical to concrete execution but with the obvious overhead
#[derive(Debug)]
pub struct NoPolicy;

// #[allow(unused)]
impl TaintPolicy for NoPolicy {

    fn check_assign(
        &mut self,
        _dst: &VarnodeData,
        _val: &(BitVec, Tag),
    ) -> Result<(), Error> { Ok(()) }

    fn check_write_mem(
        &mut self,
        _address: &Address,
        _val: (&BitVec, &Tag),
    ) -> Result<(), Error> { Ok(()) }

    fn check_cond_branch(
        &mut self,
        _opcode: &Opcode,
        _cond: &(bool, Tag),
    ) -> Result<(), Error> { Ok(()) }
    
    fn check_branch(
        &mut self,
        _opcode: &Opcode,
        _target: &(Address, Tag),
    ) -> Result<(), Error> { Ok(()) }

    fn propagate_subpiece(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _src: &(BitVec, Tag),
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propagate_int2(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _lhs: &(BitVec, Tag),
        _rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propagate_int1(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _rhs: &(BitVec, Tag)
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propagate_bool2(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _lhs: &(BitVec, Tag),
        _rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propagate_bool1(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }

    fn propagate_load(
        &mut self,
        _dst: &VarnodeData,
        _val: &(BitVec, Tag),
        _loc: &(Address, Tag),
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propagate_store(
        &mut self,
        _dst: &VarnodeData,
        _val: &(BitVec, Tag),
        _loc: &(Address, Tag),
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
}


/// the default policy of propagating everything as a bitwise or operation,
/// with no violations
#[derive(Debug)]
pub struct DefaultPolicy;

impl TaintPolicy for DefaultPolicy {
    fn check_assign(
        &mut self,
        _dst: &VarnodeData,
        _val: &(BitVec, Tag),
    ) -> Result<(), Error> { Ok(()) }

    fn check_write_mem(
        &mut self,
        _address: &Address,
        _val: (&BitVec, &Tag),
    ) -> Result<(), Error> { Ok(()) }

    /// check tag on conditional branches
    fn check_cond_branch(
        &mut self,
        _opcode: &Opcode,
        _cond: &(bool, Tag),
    ) -> Result<(), Error> { Ok(()) }

    fn check_branch(
        &mut self,
        _opcode: &Opcode,
        _target: &(Address, Tag),
    ) -> Result<(), Error> { Ok(()) }

    fn propagate_subpiece(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        src: &(BitVec, Tag),
    ) -> Result<Tag, Error> {
        Ok(src.1)
    }
    
    fn propagate_int2(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error> {
        Ok(lhs.1 | rhs.1)
    }
    
    fn propagate_int1(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error> {
        Ok(rhs.1)
    }
    
    fn propagate_bool2(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error> {
        Ok(lhs.1 | rhs.1)
    }
    
    fn propagate_bool1(
        &mut self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, Error> {
        Ok(rhs.1)
    }

    fn propagate_load(
        &mut self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
    ) -> Result<Tag, Error> {
        Ok(Tag::new()
            .with_tainted_val(loc.1.is_tainted() || val.1.is_tainted()))
    }
    
    fn propagate_store(
        &mut self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
    ) -> Result<Tag, Error> {
        Ok(Tag::new()
            .with_tainted_val(val.1.is_tainted())
            .with_tainted_loc(loc.1.is_tainted()))
    }
}