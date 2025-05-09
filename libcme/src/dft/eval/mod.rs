//! eval.rs
//! 
//! concrete pcode evaluator
use std::sync::Arc;

use thiserror::Error;

use fugue_core::prelude::*;
use fugue_core::ir::Location;
use fugue_ir::disassembly::{Opcode, VarnodeData, PCodeData};

use crate::dft::context::{self, Context};
use crate::programdb::{self, ProgramDB};
use crate::types::*;
use crate::utils::*;

use super::plugin::{self, EvaluatorPlugin};
use super::policy::{
    self,
    TaintPolicy,
};
use super::tag::{
    self,
    Tag,
};
use super::EvalPlugin;

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
    #[error(transparent)]
    ProgramDB(#[from] programdb::Error),
    #[error("policy violation: {0}")]
    Policy(anyhow::Error),
    #[error("plugin error: {0}")]
    Plugin(anyhow::Error),
}

impl From<policy::Error> for Error {
    fn from(err: policy::Error) -> Self {
        Self::Policy(err.0)
    }
}

impl From<plugin::Error> for Error {
    fn from(err: plugin::Error) -> Self {
        Self::Plugin(err.0)
    }
}

/// concrete pcode evaluator
#[derive(Debug)]
pub struct Evaluator<'policy, 'plugin> {
    pub pc: Location,
    pub pc_tag: Tag,
    pub policy: &'policy dyn TaintPolicy,
    plugin: EvaluatorPlugin<'plugin>,
}

impl<'policy, 'plugin> Default for Evaluator<'policy, 'plugin> {
    fn default() -> Self {
        Self {
            pc: Location::default(),
            pc_tag: Tag::from(tag::ACCESSED),
            policy: policy::default(),
            plugin: EvaluatorPlugin::default(),
        }
    }
}

impl<'policy, 'plugin> Evaluator<'policy, 'plugin> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_policy(policy: &'policy dyn TaintPolicy) -> Self {
        Self {
            pc: Location::default(),
            pc_tag: Tag::from(tag::ACCESSED),
            policy,
            plugin: EvaluatorPlugin::default(),
        }
    }

    pub fn add_plugin(&mut self, plugin:  Box<dyn EvalPlugin + 'plugin>) {
        self.plugin.add_plugin(plugin)
    }
}

impl<'irb, 'policy, 'backend, 'plugin> Evaluator<'policy, 'plugin> {
    #[instrument(skip_all)]
    pub fn step(&mut self,
        context: &mut Context<'backend>,
        pdb: &mut ProgramDB<'irb>,
    ) -> Result<(), Error> {
        // need to eventually decide how long a thread switch should take
        // right now there is no latency, so it'll look instantaneous
        if let Some((thread_switch, target_tag)) = context.maybe_thread_switch()? {
            // for different architectures, target may not be 32 bits, which could be an issue.
            let target = BitVec::from_u32(thread_switch.target_address.offset() as u32, 32);
            let val = (target, target_tag);
            self.policy.check_assign(context.lang().translator().program_counter(), &val)?;
            self.pc = thread_switch.target_address.into();
            self.pc_tag = target_tag;
        } else {
            let (pc, tag) = context.read_pc()?;
            self.pc = pc.into();
            self.pc_tag = tag;
        }

        // tick processor clock
        context.tick()?;

        let address = self.pc.address();

        // let insn = context.fetch(address, pdb.arena)?;
        let insn = pdb.fetch(address, context.backend())?;
        info!("pc @ {:#010x} (tag={}): {}", address.offset(), &self.pc_tag, insn.disasm_str());
        self.plugin.pre_insn_cb(&self.pc, insn.as_ref(), context)?;

        let pcode = &insn.pcode;
        let op_count = pcode.operations.len() as u32;
        let mut flow = FlowType::Fall.into();

        while address == self.pc.address() && self.pc.position() < op_count {
            let pos = self.pc.position() as usize;
            let op = &pcode.operations[pos];

            self.plugin.pre_pcode_cb(&self.pc, op, context)?;
            flow = self._evaluate(op, context)?;
            self.plugin.post_pcode_cb(&self.pc, op, context)?;

            match flow.flowtype {
                FlowType::Branch
                | FlowType::CBranch
                | FlowType::IBranch
                | FlowType::Call
                | FlowType::ICall
                | FlowType::Return
                | FlowType::Unknown => {
                    self.pc = flow.target.unwrap();
                    pdb.add_edge(address, self.pc.address(), flow.flowtype)?;
                }
                FlowType::Fall => {
                    self.pc.position += 1u32;
                }
                _ => {
                    panic!("{flow:?} is an analysis-only flow-type");
                }
            }
        }

        // update context pc value
        if matches!(flow.flowtype, FlowType::Fall) {
            self.pc = Location::from(address + pcode.len());
        }
        context.write_pc(self.pc.address(), &self.pc_tag)?;

        // handle events after pc is written
        context.process_events()?;
        
        self.plugin.post_insn_cb(&self.pc, insn.as_ref(), context)?;

        Ok(())
    }
}

impl<'irb, 'policy, 'backend, 'plugin> Evaluator<'policy, 'plugin> {
    /// evaluate a single pcode operation
    #[instrument(skip_all)]
    fn _evaluate(
        &mut self,
        operation: &PCodeData,
        context: &mut Context<'backend>,
    ) -> Result<Flow, Error> {
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

                let loc = self._read_addr(src, context)?;
                self.plugin.pre_mem_access_cb(&self.pc, &loc.0, lsz, Permission::R, context)?;
                let val = self._read_mem(&loc.0, lsz, context)?;

                let tag = self.policy.propagate_load(dst, &val, &loc)?;
                let mem_size = val.0.bytes();
                let mut value = (val.0, tag);
                self.plugin.mem_access_cb(&self.pc, &loc.0, mem_size, Permission::R, &mut value, context)?;
                let (val, tag) = value;
                self._assign(dst, val, tag, context)?;
            }
            Opcode::Store => {
                let dst = &operation.inputs[1];
                let src = &operation.inputs[2];

                let val = context.read(&src)?;
                let loc = self._read_addr(dst, context)?;

                let tag = self.policy.propagate_store(dst, &val, &loc)?;
                let mem_size = val.0.bytes();
                let mut value = (val.0, tag);
                self.plugin.pre_mem_access_cb(&self.pc, &loc.0, mem_size, Permission::W, context)?;
                self.plugin.mem_access_cb(&self.pc, &loc.0, mem_size, Permission::W, &mut value, context)?;
                let (val, tag) = value;
                self._write_mem(&loc.0, &val, &tag, context)?;
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
                return Ok(FlowType::Branch.target(target));
            }
            Opcode::CBranch => {
                let bool_val = self._read_bool(&operation.inputs[1], context)?;
                self.policy.check_cond_branch(&operation.opcode, &bool_val)?;
                if bool_val.0 {
                    let target = _absolute_loc(loc.address(), operation.inputs[0], loc.position());
                    return Ok(FlowType::Branch.target(target));
                }
            }
            Opcode::IBranch => {
                let target = self._read_addr(&operation.inputs[0], context)?;
                self.policy.check_branch(&operation.opcode, &target)?;
                return Ok(FlowType::IBranch.target(target.0.into()));
            }
            Opcode::Call => {
                let target = _absolute_loc(loc.address(), operation.inputs[0], loc.position());
                // no taint check is needed on a constant direct call (constants never tainted)
                return Ok(FlowType::Call.target(target));
            }
            Opcode::ICall => {
                let target = self._read_addr(&operation.inputs[0], context)?;
                self.policy.check_branch(&operation.opcode, &target)?;
                return Ok(FlowType::Call.target(target.0.into()));
            }
            Opcode::Return => {
                let target = self._read_addr(&operation.inputs[0], context)?;
                self.policy.check_branch(&operation.opcode, &target)?;
                return Ok(FlowType::Return.target(target.0.into()));
            }
            Opcode::CallOther => {
                let output = operation.output.as_ref();
                let inputs = &operation.inputs[..];

                let (cb_index, cb_inputs, cb_output) = get_userop_params(output, inputs);
                self.plugin.pre_userop_cb(&loc, cb_index, cb_inputs, cb_output, context)?;
                let result = context.userop(output, inputs)?;
                self.plugin.post_userop_cb(&loc, cb_index, cb_inputs, cb_output, context, &result)?;
                
                if result.is_some() {
                    return Ok(FlowType::Unknown.target(result.unwrap()));
                }
            }
            op => {
                println!("{}", operation.display(context.lang().translator()));
                return Err(Error::Unsupported(op).into())
            }
        }
        Ok(FlowType::Fall.into())
    }
}

impl<'irb, 'policy, 'backend, 'plugin> Evaluator<'policy, 'plugin> {

    fn _read_bool(&self,
        vnd: &VarnodeData,
        context: &mut Context<'backend>,
    ) -> Result<(bool, Tag), Error> {
        let (val, tag) = context.read(vnd)?;
        Ok((!val.is_zero(), tag))
    }

    fn _read_addr(&self,
        vnd: &VarnodeData,
        context: &mut Context<'backend>,
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
        context: &mut Context<'backend>,
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
        context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        let spc = context.lang()
            .translator()
            .manager()
            .default_space();
        let mem = VarnodeData::new(spc.as_ref(), address.offset(), val.bytes());
        let value = (val, tag);
        self.policy.check_write_mem(address, value)?;
        Ok(context.write(&mem, val, tag)?)
    }

    fn _assign(&self,
        vnd: &VarnodeData,
        val: BitVec,
        tag: Tag,
        context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        let val = (val, tag);
        self.policy.check_assign(vnd, &val)?;
        context.write(vnd, &val.0.cast(vnd.bits()), &val.1)
            .map_err(Error::from)
    }

    fn _subpiece(&self,
        operation: &PCodeData,
        context: &mut Context<'backend>
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

        let val = (trunc, tag);
        self.policy.propagate_subpiece(&operation.opcode, dst, &val)?;
        self._assign(dst, val.0, tag, context)
    }

    fn _apply_int2<F, G>(&self,
        operation: &PCodeData,
        cast: F,
        op: G,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>
    where
        F: Fn(BitVec, u32) -> BitVec,
        G: FnOnce(BitVec, BitVec) -> Result<BitVec, Error>
    {
        let lhs = context.read(&operation.inputs[0])?;
        let rhs = context.read(&operation.inputs[1])?;
        let dst = operation.output.as_ref().unwrap();

        let tag = self.policy
            .propagate_int2(&operation.opcode, dst, &lhs, &rhs)?;

        let size = lhs.0.bits().max(rhs.0.bits());
        let val = op(cast(lhs.0, size), cast(rhs.0, size))?;

        self._assign(dst, val.cast(dst.bits()), tag, context)
    }

    fn _apply_signed_int2<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>
    where
        F: FnOnce(BitVec, BitVec) -> Result<BitVec, Error>,
    {
        self._apply_int2(operation, |val, bits| val.signed().cast(bits), op, context)
    }

    fn _apply_unsigned_int2<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'backend>,
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
        context: &mut Context<'backend>,
    ) -> Result<(), Error>
    where
        F: Fn(BitVec) -> BitVec,
        G: FnOnce(BitVec) -> Result<BitVec, Error>,
    {
        let rhs = context.read(&operation.inputs[0])?;
        let dst = operation.output.as_ref().unwrap();

        let tag = self.policy
            .propagate_int1(&operation.opcode, dst, &rhs)?;

        let val = op(cast(rhs.0))?;

        self._assign(dst, val.cast(dst.bits()), tag, context)
    }

    fn _apply_signed_int1<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>
    where 
        F: FnOnce(BitVec) -> Result<BitVec, Error>,
    {
        self._apply_int1(operation, |val| val.signed(), op, context)
    }

    fn _apply_unsigned_int1<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>
    where
        F: FnOnce(BitVec) -> Result<BitVec, Error>,
    {
        self._apply_int1(operation, |val| val.unsigned(), op, context)
    }

    fn _apply_bool2<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>
    where
        F: FnOnce(bool, bool) -> Result<bool, Error>,
    {
        let lhs = context.read(&operation.inputs[0])?;
        let rhs = context.read(&operation.inputs[1])?;
        let dst = operation.output.as_ref().unwrap();

        let tag = self.policy
            .propagate_bool2(&operation.opcode, dst, &lhs, &rhs)?;

        let val = bool2bv(op(!lhs.0.is_zero(), !rhs.0.is_zero())?);

        self._assign(dst, val.cast(dst.bits()), tag, context)
    }

    fn _apply_bool1<F>(&self,
        operation: &PCodeData,
        op: F,
        context: &mut Context<'backend>,
    ) -> Result<(), Error>
    where 
        F: FnOnce(bool) -> Result<bool, Error>,
    {
        let rhs = context.read(&operation.inputs[0])?;
        let dst = operation.output.as_ref().unwrap();
        
        let tag = self.policy
            .propagate_bool1(&operation.opcode, dst, &rhs)?;

        let val = bool2bv(op(!rhs.0.is_zero())?);
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
    #[allow(unused)]
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

        todo!("fix this test")
    }
}