//! policy.rs
//! 
//! defining taint policies
use thiserror::Error;
use fugue_core::prelude::*;
use fugue_ir::disassembly::{
    VarnodeData,
    Opcode,
};
use super::tag::{self, Tag};

pub mod jump;


#[derive(Debug, derive_more::Display, Error)]
pub struct Error(pub(crate) anyhow::Error);

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

pub trait TaintPolicy: std::fmt::Debug {
    /// check for policy violations on varnode assignment
    fn check_assign(
        &self,
        dst: &VarnodeData,
        tag: &Tag,
    ) -> Result<(), Error>;
    /// check for policy violations on memory write
    fn check_write_mem(
        &self,
        address: &Address,
        tag: &Tag,
    ) -> Result<(), Error>;
    /// check for policy violations on branches
    fn check_branch(
        &self,
        opcode: &Opcode,
        tag: &Tag,
    ) -> Result<(), Error>;

    /// check for policy violations during subpiece operations.
    /// return error if violation detected, otherwise propagate taint
    fn propogate_subpiece(
        &self,
        opcode: &Opcode,
        dst: &VarnodeData,
        src_tag: &Tag,
    ) -> Result<Tag, Error>;
    
    /// check for policy violations during operations involving 2 integers.
    /// return error if violation detected, otherwise propagate taint
    fn propogate_int2(
        &self,
        opcode: &Opcode,
        dst: &VarnodeData,
        lhs_tag: &Tag,
        rhs_tag: &Tag,
    ) -> Result<Tag, Error>;
    
    /// check for policy violations during operations involving 2 integer.
    /// return error if violation detected, otherwise propagate taint
    fn propogate_int1(
        &self,
        opcode: &Opcode,
        dst: &VarnodeData,
        rhs_tag: &Tag,
    ) -> Result<Tag, Error>;
    
    /// check for policy violations during operations involving 2 booleans.
    /// return error if violation detected, otherwise propagate taint
    fn propogate_bool2(
        &self,
        opcode: &Opcode,
        dst: &VarnodeData,
        lhs_tag: &Tag,
        rhs_tag: &Tag,
    ) -> Result<Tag, Error>;
    
    /// check for policy violations during operations involving 1 boolean.
    /// return error if violation detected, otherwise propagate taint
    fn propogate_bool1(
        &self,
        opcode: &Opcode,
        dst: &VarnodeData,
        rhs_tag: &Tag,
    ) -> Result<Tag, Error>;

    /// check for policy violations during load operations
    /// return error if violation detected, otherwise propagate taint
    fn propagate_load(
        &self,
        dst: &VarnodeData,
        val_tag: &Tag,
        loc_tag: &Tag,
    ) -> Result<Tag, Error>;

    /// check for policy violations during store operations
    /// return error if violation detected, otherwise propagate taint
    fn propagate_store(
        &self,
        dst: &VarnodeData,
        val_tag: &Tag,
        loc_tag: &Tag,
    ) -> Result<Tag, Error>;
}



/// returns a reference to a global static default policy.
pub fn default() -> &'static DefaultPolicy {
    &DEFAULT_POLICY
}

/// returns a reference to a global static no policy struct.
pub fn no_policy() -> &'static NoPolicy {
    &NO_POLICY
}



/// a dummy policy that has no violations and no propagation
/// should be identical to concrete execution but with the obvious overhead
#[derive(Debug)]
pub struct NoPolicy;

pub static NO_POLICY: NoPolicy = NoPolicy {};

// #[allow(unused)]
impl TaintPolicy for NoPolicy {

    fn check_assign(
        &self,
        _dst: &VarnodeData,
        _tag: &Tag,
    ) -> Result<(), Error> { Ok(()) }

    fn check_write_mem(
        &self,
        _address: &Address,
        _tag: &Tag,
    ) -> Result<(), Error> { Ok(()) }

    fn check_branch(
        &self,
        _opcode: &Opcode,
        _tag: &Tag,
    ) -> Result<(), Error> { Ok(()) }

    fn propogate_subpiece(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _src_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propogate_int2(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _lhs_tag: &Tag,
        _rhs_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propogate_int1(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _rhs_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propogate_bool2(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _lhs_tag: &Tag,
        _rhs_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propogate_bool1(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        _rhs_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }

    fn propagate_load(
        &self,
        _dst: &VarnodeData,
        _val_tag: &Tag,
        _loc_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
    
    fn propagate_store(
        &self,
        _dst: &VarnodeData,
        _val_tag: &Tag,
        _loc_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(Tag::from(tag::ACCESSED))
    }
}


/// the default policy of propagating everything as a bitwise or operation,
/// with no violations
#[derive(Debug)]
pub struct DefaultPolicy;

pub static DEFAULT_POLICY: DefaultPolicy = DefaultPolicy {};

impl TaintPolicy for DefaultPolicy {
    fn check_assign(
        &self,
        _dst: &VarnodeData,
        _tag: &Tag,
    ) -> Result<(), Error> { Ok(()) }

    fn check_write_mem(
        &self,
        _address: &Address,
        _tag: &Tag,
    ) -> Result<(), Error> { Ok(()) }

    fn check_branch(
        &self,
        _opcode: &Opcode,
        _tag: &Tag,
    ) -> Result<(), Error> { Ok(()) }

    fn propogate_subpiece(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        src_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(*src_tag)
    }
    
    fn propogate_int2(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs_tag: &Tag,
        rhs_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(*lhs_tag | *rhs_tag)
    }
    
    fn propogate_int1(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(*rhs_tag)
    }
    
    fn propogate_bool2(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        lhs_tag: &Tag,
        rhs_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(*lhs_tag | *rhs_tag)
    }
    
    fn propogate_bool1(
        &self,
        _opcode: &Opcode,
        _dst: &VarnodeData,
        rhs_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(*rhs_tag)
    }

    fn propagate_load(
        &self,
        _dst: &VarnodeData,
        val_tag: &Tag,
        loc_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(Tag::new()
            .with_tainted_val(loc_tag.is_tainted() || val_tag.is_tainted()))
    }
    
    fn propagate_store(
        &self,
        _dst: &VarnodeData,
        val_tag: &Tag,
        loc_tag: &Tag,
    ) -> Result<Tag, Error> {
        Ok(Tag::new()
            .with_tainted_val(val_tag.is_tainted())
            .with_tainted_loc(loc_tag.is_tainted()))
    }
}