//! types.rs
//! 
use std::sync::Arc;

use thiserror::Error;
use flagset::flags;

use fugue_core::ir;
use fugue_ir::Address;

/// a lifted instruction
#[derive(Debug)]
pub struct Insn<'irb> {
    /// instruction's disassembly
    pub disasm: ir::Insn<'irb>,
    /// instruction's lifted pcode
    pub pcode: ir::PCode<'irb>,
}

impl<'irb> Insn<'irb> {
    pub fn disasm_str(&self) -> String {
        let mnemonic = self.disasm.mnemonic();
        let operands = self.disasm.operands();
        format!("{mnemonic:<8} {operands}")
    }
}

/// a generic register info struct
#[derive(Debug, Clone)]
pub struct RegInfo {
    /// offset relative to the register's parent base address
    pub offset: usize,
    /// access permissions
    pub perms: u8,
    /// power-on reset value if any
    pub reset: Option<u32>,
}

/// a lift error
#[derive(Debug, Error)]
pub enum LiftError {
    #[error("{0:#x?}")]
    AddressNotLifted(Address),
    #[error(transparent)]
    IR(fugue_ir::error::Error),
    #[error("{0:?}")]
    Backend(anyhow::Error),
}

/// a lift result wrapper
pub type LiftResult<'irb> = Result<Arc<Insn<'irb>>, Arc<LiftError>>;

flags! {
    /// a bitflags permission enumeration
    pub enum Permission: u8 {
        R = 0x04,
        W = 0x02,
        E = 0x01,
        RO = 0x04,
        WO = 0x02,
        RW = (Permission::R | Permission::W).bits(),
        RE = (Permission::R | Permission::E).bits(),
        WE = (Permission::W | Permission::E).bits(),
        RWE = (Permission::R | Permission::W | Permission::E).bits(),
    }
}

/// an alignment enumeration
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Alignment {
    Byte = 0b001,
    Half = 0b010,
    Word = 0b100,
    Even = 0b110,
    Any  = 0b111,
}

impl From<fugue_ir::error::Error> for LiftError {
    fn from(err: fugue_ir::error::Error) -> Self {
        Self::IR(err)
    }
}