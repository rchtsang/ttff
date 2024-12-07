//! types.rs
//! 
use fugue_core::ir;

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