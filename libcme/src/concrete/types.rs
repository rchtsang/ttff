//! types.rs
//! 
use std::sync::Arc;
use fugue_core::ir;

use super::context;

pub type LiftResult<'irb> = Result<Arc<Insn<'irb>>, context::Error>;

#[derive(Debug)]
pub struct Insn<'irb> {
    pub disasm: ir::Insn<'irb>,
    pub pcode: ir::PCode<'irb>,
}
