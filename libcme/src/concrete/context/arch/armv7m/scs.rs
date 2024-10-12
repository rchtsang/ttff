//! scs.rs

use super::*;


/// system control space
/// 
/// memory-mapped 4kb address space containing 32-bit registers for
/// configuration, status, and control [0xe000e000, 0xe000efff]
/// 
/// ARM DDI 0403E.e B3.2
#[derive(Clone)]
pub struct SysCtrlSpace {
    backing: FixedState,
}

impl AsRef<FixedState> for SysCtrlSpace {
    fn as_ref(&self) -> &FixedState {
        &self.backing
    }
}

impl AsMut<FixedState> for SysCtrlSpace {
    fn as_mut(&mut self) -> &mut FixedState {
        &mut self.backing
    }
}

impl SysCtrlSpace {
    pub fn new_from(config: SysCtrlConfig) -> Self {
        todo!("implement scs constructor")
    }
}

impl Default for SysCtrlSpace {
    fn default() -> Self {
        Self {
            backing: FixedState::new(0x1000),
        }
    }
}

/// config containing reset values for scs registers
#[derive(Debug)]
pub struct SysCtrlConfig {
    // todo
}

impl Default for SysCtrlConfig {
    fn default() -> Self {
        todo!("need to implement default scs values")
    }
}