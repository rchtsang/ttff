//! types.rs
//! 
use std::sync::Arc;
use std::ops::Range;

use thiserror::Error;
use flagset::flags;

use fugue_core::ir;
use fugue_ir::{disassembly::Opcode, Address};

/// a lift result wrapper
pub type LiftResult<'irb> = Result<Arc<Insn<'irb>>, Arc<LiftError>>;

/// thread indexing for dealing with concurrency
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EmuThread {
    Main,
    ISR { num: u32 },
}

/// control flow types
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum FlowType {
    Branch,
    CBranch,
    IBranch,
    Call,
    ICall,
    Return,
    Fall,
    Unknown,
    CallThrough,
}

impl FlowType {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Branch        => { "Branch" }
            Self::CBranch       => { "CBranch" }
            Self::IBranch       => { "IBranch" }
            Self::Call          => { "Call" }
            Self::ICall         => { "ICall" }
            Self::Return        => { "Return" }
            Self::Fall          => { "Fall" }
            Self::Unknown       => { "Unknown" }
            Self::CallThrough   => { "CallThrough" }
        }
    }
}

/// flow target
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Flow {
    pub flowtype: FlowType,
    pub target: Option<ir::Location>,
}

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

/// a mapped memory range
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MappedRange {
    Mem(Range<Address>),
    Mmio(Range<Address>),
}

impl From<fugue_ir::error::Error> for LiftError {
    fn from(err: fugue_ir::error::Error) -> Self {
        Self::IR(err)
    }
}

impl FlowType {
    pub fn target(&self, target: ir::Location) -> Flow {
        let flowtype = *self;
        let target = Some(target);
        Flow { flowtype, target }
    }

    pub fn is_branch(&self) -> bool {
        match self {
            FlowType::Fall
            | FlowType::CallThrough
            | FlowType::Unknown => { false }
            _ => { true }
        }
    }
}

impl From<FlowType> for Flow {
    fn from(flowtype: FlowType) -> Self {
        match flowtype {
            FlowType::Fall => { Self { flowtype, target: None } },
            FlowType::CallThrough => { Self { flowtype, target: None } },
            _ => { panic!("cannot convert {flowtype:?} to Flow struct") }
        }
    }
}

impl From<&Flow> for FlowType {
    fn from(flow: &Flow) -> Self {
        flow.flowtype.clone()
    }
}

impl From<Opcode> for FlowType {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::Branch    => { FlowType::Branch }
            Opcode::CBranch   => { FlowType::CBranch }
            Opcode::IBranch   => { FlowType::IBranch }
            Opcode::Call      => { FlowType::Call }
            Opcode::ICall     => { FlowType::ICall }
            Opcode::Return    => { FlowType::Return }
            Opcode::CallOther => { FlowType::Unknown }
            _                 => { FlowType::Fall }
        }
    }
}