//! eval.rs
//! 
//! concrete pcode evaluator
use std::sync::Arc;

use thiserror::Error;

use fugue_core::prelude::*;
use fugue_core::ir::Location;
use fugue_ir::disassembly::{Opcode, VarnodeData, PCodeData};

use crate::dft::context::{self, Context};
use crate::types::*;
use crate::utils::*;

use super::policy::{
    self,
    TaintPolicy,
};
use super::tag::{
    self,
    Tag,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid address: {0:x}")]
    InvalidAddress(BitVec),
    #[error("division by zero @ {0:#x?}")]
    DivideByZero(Address),
    #[error("unsupported opcode: {0:?}")]
    Unsupported(Opcode),
    #[error(transparent)]
    Context(#[from] context::Error),
    #[error(transparent)]
    Lift(#[from] Arc<LiftError>),
    #[error("policy violation: {0}")]
    Policy(anyhow::Error),
}

impl From<policy::Error> for Error {
    fn from(err: policy::Error) -> Self {
        Self::Policy(err.0)
    }
}

/// control flow types
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FlowType {
    Branch(Location),
    Call(Location),
    Return(Location),
    Fall,
}

/// concrete pcode evaluator
#[derive(Debug)]
pub struct Evaluator<'policy> {
    pub pc: Location,
    pub pc_tag: Tag,
    pub policy: &'policy dyn TaintPolicy,
}

impl<'policy> Default for Evaluator<'policy> {
    fn default() -> Self {
        Self {
            pc: Location::default(),
            pc_tag: Tag::from(tag::ACCESSED),
            policy: policy::default(),
        }
    }
}

impl<'policy> Evaluator<'policy> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_policy(policy: &'policy dyn TaintPolicy) -> Self {
        Self {
            pc: Location::default(),
            pc_tag: Tag::from(tag::ACCESSED),
            policy,
        }
    }
}

/// helper function to get absolute location
fn _absolute_loc(base: Address, vnd: VarnodeData, position: u32) -> Location {
    if !vnd.space().is_constant() {
        return Location { address: vnd.offset().into(), position: 0u32 };
    }

    let offset = vnd.offset() as i64;
    let position = if offset.is_negative() {
        position.checked_sub(offset.abs() as u32)
            .expect("negative offset from position in valid range")
    } else {
        position.checked_add(offset as u32)
            .expect("positive offset from position in valid range")
    };

    Location { address: base.into(), position }
}

impl<'irb, 'policy, 'backend> Evaluator<'policy> {
    #[instrument(skip_all)]
    pub fn step(&mut self,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error> {
        let (pc, tag) = context.read_pc()?;
        self.pc = pc.into();
        self.pc_tag = tag;
        let address = self.pc.address();

        let insn = context.fetch(address)?;
        info!("pc @ {:#010x} (tag={}): {}", address.offset(), &self.pc_tag, insn.disasm_str());
        let pcode = &insn.pcode;
        let op_count = pcode.operations.len() as u32;
        let mut target = FlowType::Fall;
        while address == self.pc.address() && self.pc.position() < op_count {
            let pos = self.pc.position() as usize;
            let op = &pcode.operations[pos];
            target = self._evaluate(op, context)?;

            match target {
                FlowType::Branch(loc)
                | FlowType::Call(loc)
                | FlowType::Return(loc) => {
                    self.pc = loc
                }
                FlowType::Fall => {
                    self.pc.position += 1u32;
                }
            }
        }
        // update context pc value
        if matches!(target, FlowType::Fall) {
            self.pc = Location::from(address + pcode.len());
        }
        context.write_pc(self.pc.address(), &self.pc_tag)?;
        Ok(())
    }
}

impl<'irb, 'policy, 'backend> Evaluator<'policy> {
    /// evaluate a single pcode operation
    #[instrument(skip_all)]
    fn _evaluate(&self,
        operation: &PCodeData,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<FlowType, Error> {
        let loc = self.pc.clone();
        debug!("{:#010x}_{}: {}", loc.address.offset(), loc.position, context.fmt_pcodeop(operation));
        debug!("    inputs: {}", context.fmt_inputs(operation)?);
        match operation.opcode {
            Opcode::Copy => {
                let (val, tag) = context.read(&operation.inputs[0])?;
                self._assign(operation.output.as_ref().unwrap(), val, tag, context)?;
            }
            Opcode::Load => {
                let dst = operation.output.as_ref().unwrap();
                let src = &operation.inputs[1];
                let lsz = dst.size();

                let (loc, loc_tag) = self._read_addr(src, context)?;
                let (val, val_tag) = self._read_mem(&loc, lsz, context)?;

                let tag = self.policy.propagate_load(dst, &val_tag, &loc_tag)?;
                self._assign(dst, val, tag, context)?;
            }
            Opcode::Store => {
                let dst = &operation.inputs[1];
                let src = &operation.inputs[2];

                let (val, val_tag) = context.read(&src)?;
                let (loc, loc_tag) = self._read_addr(dst, context)?;

                let tag = self.policy.propagate_store(dst, &val_tag, &loc_tag)?;
                self._write_mem(&loc, &val, &tag, context)?;
            }
            Opcode::IntAdd => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(lhs + rhs), context)?;
            }
            Opcode::IntSub => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(lhs - rhs), context)?;
            }
            Opcode::IntMul => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(lhs * rhs), context)?;
            }
            Opcode::IntDiv => {
                self._apply_unsigned_int2(operation, |lhs, rhs| {
                    if rhs.is_zero() {
                        Err(Error::DivideByZero(loc.address()))
                    } else {
                        Ok(lhs / rhs)
                    }
                }, context)?;
            }
            Opcode::IntSDiv => {
                self._apply_signed_int2(operation, |lhs, rhs| {
                    if rhs.is_zero() {
                        Err(Error::DivideByZero(loc.address()))
                    } else {
                        Ok(lhs / rhs)
                    }
                }, context)?;
            }
            Opcode::IntRem => {
                self._apply_unsigned_int2(operation, |lhs, rhs| {
                    if rhs.is_zero() {
                        Err(Error::DivideByZero(loc.address()))
                    } else {
                        Ok(lhs % rhs)
                    }
                }, context)?;
            }
            Opcode::IntSRem => {
                self._apply_signed_int2(operation, |lhs, rhs| {
                    if rhs.is_zero() {
                        Err(Error::DivideByZero(loc.address()))
                    } else {
                        Ok(lhs % rhs)
                    }
                }, context)?;
            }
            Opcode::IntLShift => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(lhs << rhs), context)?;
            }
            Opcode::IntRShift => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(lhs >> rhs), context)?;
            }
            Opcode::IntSRShift => {
                self._apply_signed_int2(operation, |lhs, rhs| Ok(lhs >> rhs), context)?;
            }
            Opcode::IntAnd => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(lhs & rhs), context)?;
            }
            Opcode::IntOr => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(lhs | rhs), context)?;
            }
            Opcode::IntXor => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(lhs ^ rhs), context)?;
            }
            Opcode::IntCarry => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(bool2bv(lhs.carry(&rhs))), context)?;
            }
            Opcode::IntSCarry => {
                self._apply_signed_int2(operation, |lhs, rhs| Ok(bool2bv(lhs.signed_carry(&rhs))), context)?;
            }
            Opcode::IntSBorrow => {
                self._apply_signed_int2(operation, |lhs, rhs| Ok(bool2bv(lhs.signed_borrow(&rhs))), context)?;
            }
            Opcode::IntEq => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(bool2bv(lhs == rhs)), context)?;
            }
            Opcode::IntNotEq => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(bool2bv(lhs != rhs)), context)?;
            }
            Opcode::IntLess => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(bool2bv(lhs < rhs)), context)?;
            }
            Opcode::IntSLess => {
                self._apply_signed_int2(operation, |lhs, rhs| Ok(bool2bv(lhs < rhs)), context)?;
            }
            Opcode::IntLessEq => {
                self._apply_unsigned_int2(operation, |lhs, rhs| Ok(bool2bv(lhs <= rhs)), context)?;
            }
            Opcode::IntSLessEq => {
                self._apply_signed_int2(operation, |lhs, rhs| Ok(bool2bv(lhs <= rhs)), context)?;
            }
            Opcode::IntSExt => {
                self._apply_signed_int1(operation, |val| Ok(val), context)?;
            }
            Opcode::IntZExt => {
                self._apply_unsigned_int1(operation, |val| Ok(val), context)?;
            }
            Opcode::IntNeg => {
                self._apply_signed_int1(operation, |val| Ok(-val), context)?;
            }
            Opcode::IntNot => {
                self._apply_unsigned_int1(operation, |val| Ok(!val), context)?;
            }
            Opcode::BoolNot => {
                self._apply_bool1(operation, |val| Ok(!val), context)?;
            }
            Opcode::BoolAnd => {
                self._apply_bool2(operation, |lhs, rhs| Ok(lhs & rhs), context)?;
            }
            Opcode::BoolOr => {
                self._apply_bool2(operation, |lhs, rhs| Ok(lhs | rhs), context)?;
            }
            Opcode::BoolXor => {
                self._apply_bool2(operation, |lhs, rhs| Ok(lhs ^ rhs), context)?;
            }
            Opcode::LZCount => {
                self._apply_unsigned_int1(operation, |val| {
                    Ok(BitVec::from_u32(val.leading_zeros(), val.bits()))
                }, context)?;
            }
            Opcode::PopCount => {
                self._apply_unsigned_int1(operation, |val| {
                    Ok(BitVec::from_u32(val.count_ones(), val.bits()))
                }, context)?;
            }
            Opcode::Subpiece => {
                self._subpiece(operation, context)?;
            }
            Opcode::Branch => {
                let target = _absolute_loc(loc.address(), operation.inputs[0], loc.position());
                // no taint check is needed on a constant direct branch (constants never tainted)
                return Ok(FlowType::Branch(target));
            }
            Opcode::CBranch => {
                let (bool_val, bool_tag) = self._read_bool(&operation.inputs[1], context)?;
                self.policy.check_branch(&operation.opcode, &bool_tag)?;
                if bool_val {
                    let target = _absolute_loc(loc.address(), operation.inputs[0], loc.position());
                    return Ok(FlowType::Branch(target));
                }
            }
            Opcode::IBranch => {
                let (address, tag) = self._read_addr(&operation.inputs[0], context)?;
                self.policy.check_branch(&operation.opcode, &tag)?;
                return Ok(FlowType::Branch(address.into()));
            }
            Opcode::Call => {
                let target = _absolute_loc(loc.address(), operation.inputs[0], loc.position());
                // no taint check is needed on a constant direct call (constants never tainted)
                return Ok(FlowType::Call(target));
            }
            Opcode::ICall => {
                let (address, tag) = self._read_addr(&operation.inputs[0], context)?;
                self.policy.check_branch(&operation.opcode, &tag)?;
                return Ok(FlowType::Call(address.into()));
            }
            Opcode::Return => {
                let (address, tag) = self._read_addr(&operation.inputs[0], context)?;
                self.policy.check_branch(&operation.opcode, &tag)?;
                return Ok(FlowType::Return(address.into()));
            }
            Opcode::CallOther => {
                let output = operation.output.as_ref();
                let inputs = &operation.inputs[..];
                if let Some(target) = context.userop(output, inputs)? {
                    return Ok(FlowType::Branch(target));
                }
            }
            op => {
                println!("{}", operation.display(context.lang().translator()));
                return Err(Error::Unsupported(op).into())
            }
        }
        Ok(FlowType::Fall)
    }
}

impl<'irb, 'policy, 'backend> Evaluator<'policy> {

    fn _read_bool(&self,
        vnd: &VarnodeData,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(bool, Tag), Error> {
        let (val, tag) = context.read(vnd)?;
        Ok((!val.is_zero(), tag))
    }

    fn _read_addr(&self,
        vnd: &VarnodeData,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(Address, Tag), Error> {
        let (val, tag) = context.read(vnd)?;
        val.to_u64()
            .map(Address::from)
            .ok_or_else(|| Error::InvalidAddress(val))
            .map(|address| (address, tag))
    }

    fn _read_mem(&self,
        address: &Address,
        size: usize,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(BitVec, Tag), Error> {
        let spc = context.lang()
            .translator()
            .manager()
            .default_space();
        let mem = VarnodeData::new(spc.as_ref(), address.offset(), size);
        Ok(context.read(&mem)?)
    }

    fn _write_mem(&self,
        address: &Address,
        val: &BitVec,
        tag: &Tag,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error> {
        let spc = context.lang()
            .translator()
            .manager()
            .default_space();
        let mem = VarnodeData::new(spc.as_ref(), address.offset(), val.bytes());
        self.policy.check_write_mem(address, tag)?;
        Ok(context.write(&mem, val, tag)?)
    }

    fn _assign(&self,
        vnd: &VarnodeData,
        val: BitVec,
        tag: Tag,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error> {
        self.policy.check_assign(vnd, &tag)?;
        context.write(vnd, &val.cast(vnd.bits()), &tag)
            .map_err(Error::from)
    }

    fn _subpiece(&self,
        operation: &PCodeData,
        context: &mut Context<'irb, 'backend>
    ) -> Result<(), Error> {
        let (src, tag) = context.read(&operation.inputs[0])?;
        let src_size = src.bits();
        let offset = operation.inputs[1].offset() as u32 * 8;
        let dst = operation.output.as_ref().unwrap();
        let dst_size = dst.bits();
        let trunc_size = src_size.saturating_sub(offset);
        let trunc = if dst_size > trunc_size {
            // extract high + expand
            if trunc_size >= src_size {
                src
            } else {
                src >> (src_size - trunc_size) as u32
            }
            .unsigned()
            .cast(trunc_size)
            .cast(dst_size)
        } else {
            // extract
            if offset > 0 { src >> offset as u32 } else { src }
            .unsigned()
            .cast(dst_size)
        };

        self.policy.propogate_subpiece(&operation.opcode, dst, &tag)?;
        self._assign(dst, trunc, tag, context)
    }

    fn _apply_int2<F, G>(&self,
        operation: &PCodeData,
        cast: F,
        op: G,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error>
    where
        F: Fn(BitVec, u32) -> BitVec,
        G: FnOnce(BitVec, BitVec) -> Result<BitVec, Error>
    {
        let (lhs, lhs_tag) = context.read(&operation.inputs[0])?;
        let (rhs, rhs_tag) = context.read(&operation.inputs[1])?;
        let dst = operation.output.as_ref().unwrap();

        let size = lhs.bits().max(rhs.bits());
        let val = op(cast(lhs, size), cast(rhs, size))?;

        let tag = self.policy
            .propogate_int2(&operation.opcode, dst, &lhs_tag, &rhs_tag)?;
        self._assign(dst, val.cast(dst.bits()), tag, context)
    }

    fn _apply_signed_int2<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error>
    where
        F: FnOnce(BitVec, BitVec) -> Result<BitVec, Error>,
    {
        self._apply_int2(operation, |val, bits| val.signed().cast(bits), op, context)
    }

    fn _apply_unsigned_int2<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error>
    where
        F: FnOnce(BitVec, BitVec) -> Result<BitVec, Error>,
    {
        self._apply_int2(operation, |val, bits| val.unsigned().cast(bits), op, context)
    }

    fn _apply_int1<F, G>(&self,
        operation: &PCodeData,
        cast: F,
        op: G,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error>
    where
        F: Fn(BitVec) -> BitVec,
        G: FnOnce(BitVec) -> Result<BitVec, Error>,
    {
        let (rhs, rhs_tag) = context.read(&operation.inputs[0])?;
        let dst = operation.output.as_ref().unwrap();

        let val = op(cast(rhs))?;

        let tag = self.policy
            .propogate_int1(&operation.opcode, dst, &rhs_tag)?;
        self._assign(dst, val.cast(dst.bits()), tag, context)
    }

    fn _apply_signed_int1<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error>
    where 
        F: FnOnce(BitVec) -> Result<BitVec, Error>,
    {
        self._apply_int1(operation, |val| val.signed(), op, context)
    }

    fn _apply_unsigned_int1<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error>
    where
        F: FnOnce(BitVec) -> Result<BitVec, Error>,
    {
        self._apply_int1(operation, |val| val.unsigned(), op, context)
    }

    fn _apply_bool2<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error>
    where
        F: FnOnce(bool, bool) -> Result<bool, Error>,
    {
        let (lhs, lhs_tag) = context.read(&operation.inputs[0])?;
        let (rhs, rhs_tag) = context.read(&operation.inputs[1])?;
        let dst = operation.output.as_ref().unwrap();

        let val = bool2bv(op(!lhs.is_zero(), !rhs.is_zero())?);

        let tag = self.policy
            .propogate_bool2(&operation.opcode, dst, &lhs_tag, &rhs_tag)?;

        self._assign(dst, val.cast(dst.bits()), tag, context)
    }

    fn _apply_bool1<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'irb, 'backend>,
    ) -> Result<(), Error>
    where 
        F: FnOnce(bool) -> Result<bool, Error>,
    {
        let (rhs, rhs_tag) = context.read(&operation.inputs[0])?;
        let dst = operation.output.as_ref().unwrap();

        let val = bool2bv(op(!rhs.is_zero())?);

        let tag = self.policy
            .propogate_bool1(&operation.opcode, dst, &rhs_tag)?;
        self._assign(dst, val.cast(dst.bits()), tag, context)
    }
}



#[cfg(test)]
mod test {
    use fugue_core::language::LanguageBuilder;
    use fugue_ir::disassembly::IRBuilderArena;
    // use crate::concrete::context::arch::armv7m;
    use super::*;

    #[test]
    fn test_evaluator() -> Result<(), Error> {
        let builder = LanguageBuilder::new("data/processors")
            .expect("language builder not instantiated");
        let irb = IRBuilderArena::with_capacity(0x1000);
        // let mut context = Context::new_with(&builder, &irb, None)?;

        // let size = 0x2000usize;
        // context.map_mem(0x0u64, size)?;

        // context.store_bytes(0x0u64, crate::concrete::tests::TEST_PROG_SQUARE)?;

        // context.write_pc(0x0u64)?;
        // context.write_sp(size as u64)?;

        // let mut evaluator = Evaluator::new();
        // let halt_address = Address::from(0x4u64);
        // let mut cycles = 0;
        // while evaluator.pc.address() != halt_address {
        //     let pc = evaluator.pc.address().offset();
        //     println!("pc: {pc:#x?}");
        //     evaluator.step(&mut context)?;
        //     cycles += 1;
        // }
        // assert!(cycles > 10, "instructions executed: {cycles}");

        // let r0 = context.translator().register_by_name("r0")
        //     .expect("no register named r0");
        // let retval = context.read(&r0)?;

        // assert_eq!(retval.to_i32().unwrap(), 6561, "retval: {retval:?}, cycles: {cycles}");

        Ok(())
    }
}