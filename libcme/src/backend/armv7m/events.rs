//! events.rs
//! 
//! armv7m architectural events and event processing

use crate::backend;
use super::*;

/// armv7m architecture event
/// 
/// these events can be triggered on writes to system control
/// and must be dealt with immediately with an update to the
/// context state if necessary
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    // misc events
    SetProcessorStatus(Status),

    // ICSR, SHCSR, SHPR
    ExceptionSetActive(ExceptionType, bool),
    ExceptionSetPending(ExceptionType, bool),
    ExceptionSetPriority(ExceptionType, u8),
    ExceptionEnabled(ExceptionType, bool),

    // VTOR
    VectorTableOffsetWrite(u32),

    // AIRCR
    ExternSysResetRequest,      // (SYSRESETREQ) external system reset request
    LocalSysResetRequest,       // (VECTRESET) local system reset
    ExceptionClrAllActive,      // (VECTCLRACTIVE) clear all active state info for fixed and configurable exceptions, clear ipsr to 0
    VectorKeyWrite,             // (VECTKEY) 0x05fa written to vector key register
    SetPriorityGrouping(u8),    // (PRIGROUP) set the priority grouping according to Table B1-7

    // SCR keeps state that influences execution
    SetTransitionWakupEvent(bool),  // transitions from inactive to pending are/aren't wakeup events
    SetDeepSleep(bool),             // selected sleep state is/isn't deep sleep
    SetSleepOnExit(bool),           // enter/don't enter sleep state

    // CCR keeps state that influences executon
    ThreadModeExceptionsEnabled(bool),      // (NONBASETHRDENA) allow/disallow enter/return to thread mode with active exceptions (except priority boosting)
    STIRUnprivilegedAccessAllowed(bool),    // (USERSETMPEND) allow/disallow unprivileged access to the STIR
    UnalignedAccessTrapEnabled(bool),       // (UNALIGN_TRP) enable/disable trapping on unaligned word/halfword accesses
    DivideByZeroTrapEnabled(bool),          // (DIV_0_TRP) enable/disable trapping on divide by 0
    PreciseDataAccessFaultIgnored(bool),    // (BFHFNMIGN) set lockup/ignored for precise data access faults at priorities -1 or -2
    Stack8ByteAligned(bool),                // (STKALIGN) guarantee 4-byte/8-byte stack alignment w/ SP adjustment
    DataCacheEnabled(bool),                 // (DC) enable/disable data and unified caches
    InsnCacheEnabled(bool),                 // (IC) enable/disable instruction caches
    BranchPredictionEnabled(bool),          // (BP) enable/disable program flow prediction

    // SHPR sets system handler priorities, needed by exception/interrupt handling system
    // use ExceptionSetPriority for this instead
    // SetSystemHandlerPriority { id: u8, priority: u8}, // (PRI_x in SHPR1, SHPR2, or SHPR3) set system handler x's priority level
    
    // CFSR, MMFSR, BFSR, UFSR, HFSR
    FaultStatusClr(Fault),

    // special purpose registers PRIMASK, FAULTMASK, BASEPRI, CONTROL
    
    // special events
    SEVInstructionExecuted, // the execution of a SEV instruction on any processor in the multiprocessor system

    // debug event placeholder
    Debug(DebugEvent),

    // peripheral events
    Peripheral(peripheral::Event),

    // implementation specific events
    // GenericEvent(T)
}

impl From<peripheral::Event> for Event {
    fn from(value: peripheral::Event) -> Self {
        Self::Peripheral(value)
    }
}

impl Backend {
    #[allow(unused)]
    #[instrument(skip_all)]
    pub(crate) fn handle_event(&mut self, evt: Event) -> Result<(), backend::Error> {
        info!("handling {evt:?}");
        match evt {
            Event::SetProcessorStatus(status) => {
                self.status = status;
                Ok(())
            }
            Event::ExceptionSetActive(exception_type, val) => {
                if val {
                    self.scs.set_exception_active(exception_type);
                } else {
                    self.scs.clr_exception_active(exception_type);
                }
                Ok(())
            }
            Event::ExceptionSetPending(exception_type, val) => {
                if val {
                    self.scs.set_exception_pending(exception_type);
                } else {
                    self.scs.clr_exception_pending(exception_type);
                }
                Ok(())
            }
            Event::ExceptionSetPriority(typ, pri) => {
                self.scs.set_exception_priority(typ, pri);
                Ok(())
            }
            Event::ExceptionEnabled(exception_type, val) => {
                if val {
                    self.scs.enable_exception(exception_type);
                } else {
                    self.scs.disable_exception(exception_type);
                }
                Ok(())
            }
            Event::VectorTableOffsetWrite(offset) => {
                // VTOR is used directly during preemption, so we should be able
                // to ignore this event
                Ok(())
            }
            Event::ExternSysResetRequest => {
                todo!()
            }
            Event::LocalSysResetRequest => {
                todo!()
            }
            Event::ExceptionClrAllActive => {
                // TODO: this must also clear the IPSR
                todo!()
            }
            Event::VectorKeyWrite => {
                todo!()
            }
            Event::SetPriorityGrouping(group) => {
                // right we rely on prigroup register itself for getting
                // the current prigroup, so this event doesn't need to do
                // anything
                Ok(())
            }
            Event::SetTransitionWakupEvent(val) => {
                todo!()
            }
            Event::SetDeepSleep(val) => {
                todo!()
            }
            Event::SetSleepOnExit(val) => {
                todo!()
            }
            Event::ThreadModeExceptionsEnabled(val) => {
                todo!()
            }
            Event::STIRUnprivilegedAccessAllowed(val) => {
                todo!()
            }
            Event::UnalignedAccessTrapEnabled(val) => {
                todo!()
            }
            Event::DivideByZeroTrapEnabled(val) => {
                todo!()
            }
            Event::PreciseDataAccessFaultIgnored(val) => {
                todo!()
            }
            Event::Stack8ByteAligned(val) => {
                todo!()
            }
            Event::DataCacheEnabled(val) => {
                todo!()
            }
            Event::InsnCacheEnabled(val) => {
                todo!()
            }
            Event::BranchPredictionEnabled(val) => {
                todo!()
            }
            Event::FaultStatusClr(fault) => {
                todo!("need to implement fault handling for this to make sense")
                // go through all the possible CFSR error conditions 
                // some of them trap conditionally. rather annoying to implement.
            }
            Event::SEVInstructionExecuted => {
                // do nothing with this for now
                // see SEV A7.7.129
                Ok(())
            }
            Event::Debug(_evt) => {
                todo!()
            }
            Event::Peripheral(evt) => {
                let mut nvicregs = self.scs.nvic_regs_mut();
                // since scs exception state calls don't update memory locations,
                // we need to do that manually for each of these.
                // this might be worth a refactor at some point...
                match evt {
                    peripheral::Event::EnableInterrupt { int_num } => {
                        let n = ((int_num + 16) / 32) as u8;
                        let i = (int_num + 16) % 32;
                        let iser = nvicregs.get_iser(n).setena();
                        let icer = nvicregs.get_icer(n).clrena();
                        nvicregs.get_iser_mut(n)
                            .set_setena(iser | (1 << i));
                        nvicregs.get_icer_mut(n)
                            .set_clrena(icer | (1 << i));
                        let typ = ExceptionType::from(int_num + 16);
                        self.scs.enable_exception(typ);
                        Ok(())
                    }
                    peripheral::Event::DisableInterrupt { int_num } => {
                        let n = ((int_num + 16) / 32) as u8;
                        let i = (int_num + 16) % 32;
                        let iser = nvicregs.get_iser(n).setena();
                        let icer = nvicregs.get_icer(n).clrena();
                        nvicregs.get_iser_mut(n)
                            .set_setena(iser & !(1 << i));
                        nvicregs.get_icer_mut(n)
                            .set_clrena(icer & !(1 << i));
                        let typ = ExceptionType::from(int_num + 16);
                        self.scs.disable_exception(typ);
                        Ok(())
                    }
                    peripheral::Event::FireInterrupt { int_num } => {
                        let n = ((int_num + 16) / 32) as u8;
                        let i = (int_num + 16) % 32;
                        let ispr = nvicregs.get_ispr(n).setpend();
                        let icpr = nvicregs.get_icpr(n).clrpend();
                        nvicregs.get_ispr_mut(n)
                            .set_setpend(ispr | (1 << i));
                        nvicregs.get_icpr_mut(n)
                            .set_clrpend(icpr | (1 << i));
                        let typ = ExceptionType::from(int_num + 16);
                        self.scs.set_exception_pending(typ);
                        Ok(())
                    }
                }
            }
        }
    }
}