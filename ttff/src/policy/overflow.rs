//! overflow.rs
//! 
//! tainted integer overflow policy as described in "All You Ever Wanted to 
//! Know..." by Schwartz, Avgerinos, and Brumley, 2010
//! 
//! a helpful resource: 
//! https://www.gnu.org/software/autoconf/manual/autoconf-2.63/html_node/Integer-Overflow.html

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
use libcme::dtt::{
    tag::Tag,
    policy::*,
};

/// control flow policy violations
#[derive(Clone, Error, Debug)]
pub enum PolicyViolation {
    #[error("a tainted value caused an integer overflow")]
    TaintedOverflow,
    #[error("a tainted value caused an integer underflow")]
    TaintedUnderflow,
}

/// a control flow integrity policy to catch tainted PC writes
pub struct TaintedOverflowPolicy {
    pub lang: Arc<Language>,
}

impl TaintedOverflowPolicy {
    pub fn new_with(lang: Arc<Language>) -> Self {
        Self { lang }
    }
}

impl TaintPolicy for TaintedOverflowPolicy {

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
        opcode: &Opcode,
        _dst: &VarnodeData,
        lhs: &(BitVec, Tag),
        rhs: &(BitVec, Tag),
    ) -> Result<Tag, policy::Error> {
        if !lhs.1.is_tainted() && !rhs.1.is_tainted() {
            // neither is tainted, return a clean tag immediately
            return Ok(lhs.1 | rhs.1);
        }
        let signed = lhs.0.is_signed();
        assert_eq!(signed, false, 
            "overflow checks only works on unsigned");
        assert_eq!(signed, rhs.0.is_signed(),
            "lhs and rhs must both be unsigned");
        let num_bits = lhs.0.bits();
        let max_value = BitVec::max_value_with(num_bits, signed);
        // checks are based on https://stackoverflow.com/questions/199333
        match opcode {
            Opcode::IntAdd if lhs.0 > max_value - rhs.0.clone() => {
                Err(policy::Error::from(PolicyViolation::TaintedOverflow))
            }
            // Opcode::IntSub if lhs.0 < rhs.0 => {
            //     // excluded since `cmp` instructions rely on underflow behavior
            //     Err(policy::Error::from(PolicyViolation::TaintedUnderflow))
            // }
            Opcode::IntMul if {
                // based on responses from https://stackoverflow.com/questions/1815367
                // do mul in larger bitvec and check result
                // underlying implementation of bitvec makes this probably something
                // like long multiplication check when it's backed by a bigint
                let big_lhs = lhs.0.clone().cast(num_bits * 2);
                let big_rhs = rhs.0.clone().cast(num_bits * 2);
                let result = big_lhs * big_rhs;
                !(result >> num_bits).is_zero()
            } => {
                Err(policy::Error::from(PolicyViolation::TaintedOverflow))
            }
            // Opcode::IntDiv if {
            //     // check for divide by -1 assuming 2's complement
            //     // note that we cannot check this due to current evaluator's implementation
            //     let neg_one = BitVec::from_i32(-1, num_bits);
            //     let min_signed_value = BitVec::min_value_with(num_bits, true)
            //         .unsigned();
            //     rhs.0 == neg_one && lhs.0 == min_signed_value
            // } => {
            //     Err(policy::Error::from(PolicyViolation::TaintedOverflow))
            // }
            _ => { Ok(lhs.1 | rhs.1) }
        }
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
    fn propagate_load<'a>(
        &mut self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
        _ctx: &dtt::Context<'a>,
    ) -> Result<Tag, policy::Error> {
        Ok(Tag::new()
            .with_tainted_val(val.1.is_tainted())
            .with_tainted_loc(loc.1.is_tainted()))
    }
    
    /// a stored value is considered tainted if it came from a tainted
    /// source, or if the pointer used to store it was tainted.
    fn propagate_store<'a>(
        &mut self,
        _dst: &VarnodeData,
        val: &(BitVec, Tag),
        loc: &(Address, Tag),
        _ctx: &dtt::Context<'a>,
    ) -> Result<Tag, policy::Error> {
        Ok(Tag::new()
            .with_tainted_val(val.1.is_tainted())
            .with_tainted_loc(loc.1.is_tainted()))
    }
}

impl std::fmt::Debug for TaintedOverflowPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arch = self.lang.translator().architecture();
        write!(f, "TaintedOverflowPolicy {{ {arch} }}")
    }
}

impl From<PolicyViolation> for policy::Error {
    fn from(value: PolicyViolation) -> Self {
        policy::Error::from(anyhow::Error::from(value))
    }
}