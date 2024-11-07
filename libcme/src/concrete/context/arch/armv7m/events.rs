//! events.rs
//! 
//! armv7m architectural events and event processing
use crate::utils::*;

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

    // ICSR and SHCSR
    ExceptionSetActive(ExceptionType, bool),
    ExceptionSetPending(ExceptionType, bool),
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
    SetSystemHandlerPriority { id: u8, priority: u8}, // (PRI_x in SHPR1, SHPR2, or SHPR3) set system handler x's priority level
    
    // CFSR, MMFSR, BFSR, UFSR, HFSR
    FaultStatusClr(Fault),

    // special purpose registers PRIMASK, FAULTMASK, BASEPRI, CONTROL
    
    // special events
    SEVInstructionExecuted, // the execution of a SEV instruction on any processor in the multiprocessor system

    // debug event placeholder
    Debug(DebugEvent),

    // implementation specific events
    // GenericEvent(T)
}

impl<'irb> Context<'irb> {
    fn _process_events(&mut self) -> Result<(), context::Error> {
        while let Some(evt) = self.events.pop_front() {
            self._handle_event(evt).map_err(Into::<context::Error>::into)?;
        }
        Ok(())
    }

    #[allow(unused)]
    fn _handle_event(&mut self, evt: Event) -> Result<(), Error> {
        match evt {
            Event::SetProcessorStatus(status) => {
                self.status = status;
                Ok(())
            }
            Event::ExceptionSetActive(exception_type, val) => {
                todo!()
            }
            Event::ExceptionSetPending(exception_type, val) => {
                todo!()
            }
            Event::ExceptionEnabled(exception_type, val) => {
                todo!()
            }
            Event::VectorTableOffsetWrite(offset) => {
                todo!()
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
                todo!()
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
            Event::SetSystemHandlerPriority{ id, priority } => {
                todo!()
            }
            Event::FaultStatusClr(fault) => {
                todo!()
            }
            Event::SEVInstructionExecuted => {
                todo!()
            }
            Event::Debug(_evt) => {
                todo!()
            }
        }
    }
}

impl<'irb> Context<'irb> {
    /// returns true if the event is a WFE wakeup event based on current
    /// processor state (see B1.5.18)
    #[instrument]
    pub fn is_wfe_wakeup_evt(&self, evt: &Event) -> bool {
        match evt {
            // execution of a SEV instruction on any processor in a multiprocessor system
            Event::SEVInstructionExecuted => {
                true
            }
            // any exception entering the pending state if SCR.SEVONPEND is set.
            Event::ExceptionSetPending(_typ, true)
            if self.scs.get_scr().sevonpend() => {
                true
            }
            // an asynchronous exception at a priority that preempts any currently
            // active exceptions
            Event::ExceptionSetActive(typ, true)
            | Event::ExceptionSetPending(typ, true) => {
                let Some(exception) = self.scs.nvic.get_exception(typ) else {
                    warn!("processor may be in inconsistent state: no exception registered for exception: {typ:?}");
                    return false;
                };
                let priority = exception.priority;
                
                priority < self.scs.nvic.current_priority(&self.scs)
            }
            // a debug event with debug enabled
            Event::Debug(_) => {
                let offset = DebugRegType::DHCSR.offset();
                let reg_ref = self.scs.get_reg_ref(offset).unwrap();
                let dbg_ref = DebugRegRef::try_from(reg_ref).unwrap();
                let dhcsr: &DHCSR = dbg_ref.try_into().unwrap();
                dhcsr.s_halt()
            }
            _ => { false }
        }
    }

    /// returns true if the event is a WFI wakeup event based on current
    /// processor state (see B1.5.19)
    pub fn is_wfi_wakeup_evt(&self, evt: &Event) -> bool {
        match evt {
            // reset
            Event::ExternSysResetRequest
            | Event::LocalSysResetRequest => {
                true
            }
            // asynchronous exception at a priority that would preempt any 
            // currently active exception if PRIMASK were 0, (actual value of
            // PRIMASK is ignored)
            Event::ExceptionSetActive(typ, true)
            | Event::ExceptionSetPending(typ, true) => {
                let Some(exception) = self.scs.nvic.get_exception(typ) else {
                    warn!("processor may be in inconsistent state: no exception registered for exception: {typ:?}");
                    return false;
                };
                let priority = exception.priority;
                
                let vecactive = self.scs.get_icsr().vectactive();
                let current_typ: ExceptionType = vecactive.into();
                let Some(current_excp) = self.scs.nvic.get_exception(&current_typ) else {
                    panic!("processor is in inconsistent state: no exception registered for exception: {current_typ:?}");
                };
                let unmasked_current_priority = current_excp.priority;

                priority < unmasked_current_priority
            }
            // a debug event with debug enabled
            Event::Debug(_) => {
                let offset = DebugRegType::DHCSR.offset();
                let reg_ref = self.scs.get_reg_ref(offset).unwrap();
                let dbg_ref = DebugRegRef::try_from(reg_ref).unwrap();
                let dhcsr: &DHCSR = dbg_ref.try_into().unwrap();
                dhcsr.s_halt()
            }
            // other implementation-defined events

            _ => { false }
        }
    }
}