//! dcb.rs
//! 
//! debug control block

use derive_more::From;
use bitfield_struct::bitfield;

use crate::types::RegInfo;
use crate::backend;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebugEvent {
    // todo
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DebugRegType {
    /// debug halting control and status register
    DHCSR,
    /// debug core register selector register
    DCRSR,
    /// debug core register data register
    DCRDR,
    /// debug exception and monitor control register
    DEMCR,
}

impl DebugRegType {
    pub fn lookup_offset(offset: usize) -> Option<DebugRegType> {
        match offset {
            0xdf0 => { Some(DebugRegType::DHCSR) }
            0xdf4 => { Some(DebugRegType::DCRSR) }
            0xdf8 => { Some(DebugRegType::DCRDR) }
            0xdfc => { Some(DebugRegType::DEMCR) }
            _ => { None }
        }
    }

    /// returns register's address
    pub fn address(&self) -> Address {
        (0xe000e000 + self.offset() as u32).into()
    }

    /// returns byte offset into system control space of the
    /// debug register type
    pub fn offset(&self) -> usize {
        self._data().offset
    }

    /// returns access permissions of debug register type
    pub fn permissions(&self) -> u8 {
        self._data().perms
    }

    /// returns debug register reset value
    pub fn reset_value(&self) -> Option<u32> {
        self._data().reset
    }

    fn _data(&self) -> &'static RegInfo {
        match self {
            DebugRegType::DHCSR => { &RegInfo { offset: 0xdf0, perms: 0b110, reset: None } }
            DebugRegType::DCRSR => { &RegInfo { offset: 0xdf4, perms: 0b010, reset: None } }
            DebugRegType::DCRDR => { &RegInfo { offset: 0xdf8, perms: 0b110, reset: None } }
            DebugRegType::DEMCR => { &RegInfo { offset: 0xdfc, perms: 0b110, reset: None } }
        }
    }
}

/// controls halting debug.
/// constraints:
/// - modifying C_STEP or C_MASKINTS in non-debug state with halting debug enabled
///   is unpredictable. halting debug is enabled when C_DEBUGEN is set to 1.
///   the processor is in non-debug when S_HALT reads as 0.
/// - when C_DEBUGEN is set to 0, processor ignores values of all other bits in
///   this register.
/// - DHCSR is typically accessed by a debugger, through the DAP. software sunning on
///   the processor can update all fields in this register except C_DEBUGEN.
/// - access to the DHCSR from software running on the processor is IMPLEMENTATION-
///   DEFINED.
/// - see C1-695 for more info.
/// 
/// note that this struct is defined for the read configuration.
/// on write, the upper 16 bits are DBGKEY and handled directly in write_bytes function.
/// 
/// see C1.6.2
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DHCSR {
    /// halting debug enable bit.
    /// (0 = disabled, 1 = enabled)
    /// 
    /// if a debugger writes to DHCSR to change the value of this bit from 0 to 1, 
    /// it must also write 0 to the C_MASKINTS bit, otherwise behavior is unpredictable.
    /// this bit can only be written by the DAP, ignores writes from software.
    /// this bit is 0 after power-on reset.
    #[bits(1)]
    pub c_debugen: bool,
    /// processor halt bit, effects of writes to this bit are:
    /// (0 = processor leaves debug state if in debug, 1 = halt the processor)
    /// 
    /// see Table C1-9 for effects when in debug state.
    /// if C_DEBUGEN is 0, this bit value is UNKNOWN.
    /// this bit is UNKNOWN after power-on reset, and 0 after local reset.
    #[bits(1)]
    pub c_halt: bool,
    /// processor step bit. write effects:
    /// (0 = no effect, 1 = single-step enabled)
    /// 
    /// see table C1-9.
    /// When C_DEBUGEN is 0, bit value is UNKNOWN.
    /// bit is UNKNOWN after power-on reset.
    #[bits(1)]
    pub c_step: bool,
    /// when debug is enabled, debugger can write to this bit to mask PendSV,
    /// SysTick, and external configurable interrupts.
    /// (0 = no mask, 1 = mask PendSV, SysTick, and external configurable interrupts)
    ///
    /// effects of any attempt to change this bit is UNPREDICTABLE unless:
    /// - C_HALT = 1 before write
    /// - write also writes 1 to C_HALT when writing to C_MASKINTS
    /// a single write to DHCSR cannot set C_HALT to 0 and change C_MASKINTS.
    /// 
    /// this bit does not affect NMI.
    /// value UNKNOWN when C_DEBUGEN is 0 and after power-on reset.
    /// 
    /// see C1-695.
    #[bits(1)]
    pub c_maskints: bool, 
    #[bits(1)]
    __: bool,
    /// allow imprecise entry to debug state.
    /// on write: (0 = no action, 1 = allow imprecise entry)
    /// e.g. forcing any stalled load or store to complete may cause imprecise entry.
    /// 
    /// setting bit to 1 is UNPREDICTABLE unless C_DEBUGEN and C_HALT are also written to 1.
    /// writing 1 makes state of memory system UNPREDICTABLE. If debugger writes 1 to this 
    /// bit, it must reset the processor before leaving debug state.
    /// 
    /// notes:
    /// - debugger can write to DHCSR to clear bit to 0, but this does not
    ///   remove the UNPREDICTABLE state of memory.
    /// - architecture does not guarantee that setting this bit to 1 will force
    ///   entry to debug state.
    /// - arm strongly recommends that a value of 1 is never written when in debug state.
    /// 
    /// power-on reset sets this bit to 0.
    #[bits(1)]
    pub c_snapstall: bool,
    #[bits(10)]
    __: u32,
    /// a handshake flag for transfers thorugh the DCRDR.
    /// any write to DCRSR clears this bit to 0.
    /// completion of DCRDR transfer then sets bit to 1.
    /// (0 = DCRDR is written, transfer incomplete, 1 = transfer complete)
    /// 
    /// valid only when processor in debug state, otherwise UNKNOWN.
    /// read-only.
    #[bits(1)]
    pub s_regrdy: bool,
    /// indicates whether the processor in debug state.
    /// (0 = non-debug state, 1 = debug state)
    /// read-only.
    #[bits(1)]
    pub s_halt: bool,
    /// indicates whether processor is sleeping.
    /// (0 = not sleeping, 1 = sleeping)
    /// 
    /// debugger must set the C_HALT bit to 1 to gain control, or wait for
    /// an interrupt or other wakeup event to wakeup the system.
    /// read-only.
    #[bits(1)]
    pub s_sleep: bool,
    /// indicates whether processor is locked up because of unrecoverable exception
    /// (0 = not locked up, 1 = running but locked up)
    /// see Unrecoverable exception cases on B1-555.
    /// 
    /// bit can only be read as 1 by remote debugger, using DAP.
    /// bit clears to 0 when processor enters debug state.
    /// read-only.
    #[bits(1)]
    pub s_lockup: bool,
    #[bits(4)]
    __: u8,
    /// set to 1 every time processor retires one or more instructions:
    /// (0 = no instruction retired since last read to DHCSR, 1 = at least 1 retired).
    /// 
    /// the architecture does not define precisely when this bit is set to 1.
    /// it requires only that this happen periodically in non-ebug state to indicate
    /// that software execution is processing.
    /// 
    /// this is a sticky bit, clears to 0 on read of DHCSR.
    /// read-only.
    #[bits(1)]
    pub s_retire_st: bool,
    /// indicates whether the processor has been reset since the last read of DHCSR:
    /// (0 = no reset since last read, 1 = at least 1 reset).
    /// 
    /// this is a sticky bit, clears to 0 on read of DHCSR.
    /// read-only.
    #[bits(1)]
    pub s_reset_st: bool,
    #[bits(6)]
    __: u8,
}

// DDI0403E C1-705 ////////////////////////////////////////////////////////////////////////////////
// Use of DCRSR and DCRDR
// 
// In Debug state, writing to DCRSR clears the DHCSR.S_REGRDY bit to 0,
// and the processor then sets the bit to 1 when the transfer between the DCRDR and
// the Arm core register, special-purpose register, or Floating-point Extension register 
// completes. For more information about the DHCSR.S_REGRDY bit see  DHCSR on page C1-700.
// 
// This means that:
// - To transfer a data word to an Arm core register, special-purpose register, or 
//   Floating-point Extension register, a debugger:
//   1.  Writes the required word to DCRDR.
//   2.  Writes to the DCRSR, with the REGSEL value indicating the required register, 
//       and the REGWnR bit as 1 to indicate a write access.
//       This write clears the DHCSR S_REGRDY bit to 0.
//   3.  If required, polls DHCSR until DHCSR.S_REGRDY reads-as-one. This shows that 
//       the processor has transferred the DCRDR value to the selected register.
// 
// - To transfer a data word from an Arm core register, special-purpose register, or 
//   Floating-point Extension register, a debugger:
//   1.  Writes to the DCRSR, with the REGSEL value indicating the required register, and 
//       the REGWnR bit as 0 to indicate a read access.
//       This write clears the DHCSR.S_REGRDY bit to 0.
//   2.  Polls DHCSR until DHCSR.S_REGRDY reads-as-one. This shows that the processor 
//       has transferred the value of the selected register to DCRDR.
//   3.  Reads the required value from DCRDR.
// 
// When using this mechanism to write to the Arm core registers, special-purpose registers, 
// or Floating-point Extension registers:
// - All bits of the xPSR registers are fully accessible. The effect of writing an illegal 
//   value is UNPREDICTABLE.
// 
// Note: This differs from the behavior of MSR and MRS instruction accesses to the xPSR, 
//       where some bits RAZ, and some bits are ignored on writes.
// 
// - The debugger can write to the EPSR.IT bits. If it does this, it must write a value 
//   consistent with the instruction to be executed on exiting Debug state, otherwise 
//   instruction execution will be UNPREDICTABLE. See ITSTATE on page A7-179 for 
//   more information. The IT bits must be zero on exit from Debug state if the instruction 
//   indicated by DebugReturnAddress is outside an IT block.
// - The debugger can write to the EPSR.ICI bits, and on exiting Debug state any interrupted 
//   LDM or STM instruction will use these new values. Clearing the ICI bits to zero will 
//   cause the interrupted LDM or STM instruction to restart instead of continue. For more 
//   information see Exceptions in Load Multiple and Store Multiple operations on page B1-543.
// - The debugger can write to the DebugReturnAddress, and on exiting Debug state the 
//   processor starts executing from this updated address. The debugger must ensure the EPSR.IT 
//   bits and EPSR.ICI bits are consistent with the new DebugReturnAddress, as described in 
//   this list.
// - The debugger can always set FAULTMASK to 1, and doing so might cause unexpected behavior 
//   on exit from Debug state.
// 
// Note: An MSR instruction cannot set FAULTMASK to 1 when the execution priority is -1 or 
//       higher, see MSR on page B5-677.
// ////////////////////////////////////////////////////////////////////////////////////////////////

/// provides debug access to arm core registers, special purpose registers, and
/// floating-point extension registers. a write to DCRSR specifies the register to 
/// transfer, whether the tansfer is read or write, and starts transfer.
/// 
/// only accessible in debug state.
/// see C1-705.
/// 
/// note: when processor in debug state, debugger must preserve exception number
/// bits in the IPSR, otherwise behavior is UNPREDICTABLE
/// 
/// write-only.
/// 
/// see C1.6.3
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DCRSR {
    /// specifies ARM core register, special purpose register, or FP register
    /// to transfer:
    /// - [0b0000000, 0b0001100]: R0 - R12
    /// - 0b0001101: current SP
    /// - 0b0001110: LR
    /// - 0b0001111: DebugReturnAddress
    /// - 0b0010000: xPSR
    /// - 0b0010001: main SP
    /// - 0b0010010: process SP
    /// - 0b0010100: CONTROL, FAULTMASK, BASEPRI, PRIMASK (packed with leading zeros)
    /// - 0b0100001: FPSCR
    /// - [0b1000000, 0b1011111]: S0 - S31 (FP registers)
    /// - all other values reserved
    /// if processor does not implement FP extension, bits [6:5] are reserved
    #[bits(7)]
    pub regsel: u8,
    #[bits(9)]
    __: u16,
    /// specifies access type for transfer
    /// (0 = read, 1 = write)
    #[bits(1)]
    pub regwnr: bool,
    #[bits(15)]
    __: u16,
}

/// provides debug access to arm core registers, special-purpose registers, and 
/// fp extension registers. DCRDR is data register for accesses.
/// 
/// note: on its own, it provides a message passing resource between an external 
///       debugger and a debug agent running on the processor.
///       architecture does not define any handshake mechanism for this.
/// 
/// reset value is UNKNOWN.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DCRDR {
    /// data temporary cache for reading/writing registers.
    /// the value is UNKOWN on reset or while S_REGRDY is valid and 0.
    #[bits(32)]
    pub dbgtmp: u32,
}

/// manages vector catch behavior and debug monitor handling when debugging
/// - bits[23:16] provide DebugMonitor exception control
/// - bits[15:0] provide Debug state, Halting, debug, control.
///   processor ignores these values if C_DEBUGEN is 0.
/// 
/// see C1-699.
/// power-on reset resets all register bits to zero.
/// local reset only resets bits[19:16] related to DebugMonitor to zero.
/// (B1-530 defines local reset.)
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DEMCR {
    /// enable reset vector catch. causes local reset to halt a 
    /// running system.
    /// (0 = disabled, 1 = enabled)
    #[bits(1)]
    pub vc_corereset: bool,
    #[bits(3)]
    __: u8,
    /// enable halting debug trap on MemManage exception
    /// (0 = trap disabled, 1 = trap enabled)
    #[bits(1)]
    pub vc_mmerr: bool,
    /// enable halting debug trap on UsageFault caused by coprocessor access
    /// (0 = trap disabled, 1 = trap enabled)
    #[bits(1)]
    pub vc_nocperr: bool,
    /// enable halting debug trap on UsageFault caused by checking error
    /// (0 = trap disabled, 1 = trap enabled)
    #[bits(1)]
    pub vc_chkerr: bool,
    /// enable halting debug trap on UsageFault caused by state information error
    /// (0 = trap disabled, 1 = trap enabled)
    #[bits(1)]
    pub vc_staterr: bool,
    /// enable halting debug trap on BusFault exception
    /// (0 = trap disabled, 1 = trap enabled)
    #[bits(1)]
    pub vc_buserr: bool,
    /// enable halting debug trap on fault occuring during exception entry or return
    /// (0 = trap disabled, 1 = trap enabled)
    #[bits(1)]
    pub vc_interr: bool,
    /// enable halting debug trap on HardFault exception
    /// (0 = trap disabled, 1 = trap enabled)
    #[bits(1)]
    pub vc_harderr: bool,
    #[bits(5)]
    __: u8,
    /// enable DebugMonitor exception
    /// (0 = disabled, 1 = enabled)
    /// if C_DEBUGEN = 1, processor ignores this bit.
    #[bits(1)]
    pub mon_en: bool,
    /// sets or clears pending state of DebugMonitor exception:
    /// (0 = clear to not pending, 1 = set pending).
    /// 
    /// when pending, it becoes active subject to exception priority rules.
    /// debugger can use this bit to wakeup monitor with DAP.
    /// 
    /// effect of setting to 1 is not affected by MON_EN.
    /// debugger can set MON_PEND to 1 and force processor to take exception
    /// event if MON_EN = 0.
    #[bits(1)]
    pub mon_pend: bool,
    /// feature ignored when MON_EN = 0.
    /// otherwise: (0 = do not step processor, 1 = step processor)
    /// setting to 1 makes step request pending.
    /// changing this bit at execution priority lower than priority of 
    /// DebugMonitor is UNPREDICTABLE.
    #[bits(1)]
    pub mon_step: bool,
    /// DebugMonitor sempahore bit. processor does not use this bit.
    /// the monitor software defines meaning and use of this bit.
    #[bits(1)]
    pub mon_req: bool,
    #[bits(4)]
    __: u8,
    /// global enable for all DWT and ITM features:
    /// (0 = both disabled, 1 = both enabled)
    /// if DWT and ITM not implemented, bit is UNK/SBZP.
    /// 
    /// when set to 0: DWT and ITM registers return UNKNOWN on reads. 
    /// whether they are write ignored is IMPLEMENTATION DEFINED
    /// 
    /// setting to 0 may not stop all events. all DWT and ITM feature enable
    /// bits must set to 0 before setting this bit to 0 to ensure all events
    /// are stopped.
    /// 
    /// effect of this bit on TPIU, ETM, or other trace components is
    /// IMPLEMENTATION DEFINED.
    #[bits(1)]
    pub trcena: bool,
    #[bits(7)]
    __: u8,
}


/// debug wrapper struct
/// 
/// used as a temporary wrapper struct to interact with the debug registers
/// in the scs and perform debug-related operations
pub struct DebugRegs<'a> {
    backing: &'a mut [u32; 0x3c0],
}

impl<'a> DebugRegs<'a> {
    pub fn new(backing: &'a mut [u32; 0x3c0]) -> Self {
        Self { backing }
    }

    /// returns true if in debug state
    pub fn get_debug_state(&self) -> bool {
        self.get_dhcsr().s_halt()
    }

    /// perform an event-triggering read of debug register bytes
    pub fn read_bytes(&mut self,
        offset: usize,
        dst: &mut [u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), backend::Error> {
        todo!()
    }

    /// perform an event-triggering write to debug register bytes
    pub fn write_bytes(&mut self,
        offset: usize,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), backend::Error> {
        todo!()
    }
}


impl<'a> DebugRegs<'a> {
    pub fn get_dhcsr(&self) -> &DHCSR {
        let offset = DebugRegType::DHCSR.offset() / 4;
        unsafe { &*(&self.backing[offset] as *const u32 as *const DHCSR) }
    }

    pub fn get_dcrsr(&self) -> &DCRSR {
        let offset = DebugRegType::DCRSR.offset() / 4;
        unsafe { &*(&self.backing[offset] as *const u32 as *const DCRSR) }
    }

    pub fn get_dcrdr(&self) -> &DCRDR {
        let offset = DebugRegType::DCRDR.offset() / 4;
        unsafe { &*(&self.backing[offset] as *const u32 as *const DCRDR) }
    }

    pub fn get_demcr(&self) -> &DEMCR {
        let offset = DebugRegType::DEMCR.offset() / 4;
        unsafe { &*(&self.backing[offset] as *const u32 as *const DEMCR) }
    }

    pub fn get_dhcsr_mut(&mut self) -> &mut DHCSR {
        let offset = DebugRegType::DHCSR.offset() / 4;
        unsafe { &mut *(&mut self.backing[offset] as *mut u32 as *mut DHCSR) }
    }

    pub fn get_dcrsr_mut(&mut self) -> &mut DCRSR {
        let offset = DebugRegType::DCRSR.offset() / 4;
        unsafe { &mut *(&mut self.backing[offset] as *mut u32 as *mut DCRSR) }
    }

    pub fn get_dcrdr_mut(&mut self) -> &mut DCRDR {
        let offset = DebugRegType::DCRDR.offset() / 4;
        unsafe { &mut *(&mut self.backing[offset] as *mut u32 as *mut DCRDR) }
    }

    pub fn get_demcr_mut(&mut self) -> &mut DEMCR {
        let offset = DebugRegType::DEMCR.offset() / 4;
        unsafe { &mut *(&mut self.backing[offset] as *mut u32 as *mut DEMCR) }
    }

}