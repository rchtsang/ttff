//! helpers.rs
//! 
//! implementation of helper functions that require full backend context

use super::*;

impl Backend {
    pub fn get_main_sp(&self) -> Result<Address, super::Error> {
        let val = if self.is_sp_main() {
            self.read_sp()
                .map_err(|_| super::Error::System("failed to read sp"))?
                .offset()
        } else {
            assert!(self.main_sp.is_some(), "main sp must be initialized");
            self.main_sp.unwrap() as u64
        };
        Ok(Address::from(val))
    }

    pub fn get_proc_sp(&self) -> Result<Address, super::Error> {
        let val = if !self.is_sp_main() {
            self.read_sp()
                .map_err(|_| super::Error::System("failed to read sp"))?
                .offset()
        } else {
            self.proc_sp.unwrap_or(DEFAULT_PROC_SP) as u64
        };
        Ok(Address::from(val))
    }

    /// exception entry (see B1.5.6)
    #[instrument(skip_all)]
    pub fn exception_entry(&mut self, excp_typ: ExceptionType) -> Result<(), super::Error> {
        self.push_stack(excp_typ)?;
        self.exception_taken(excp_typ)
    }

    /// push stack variables on exception entry (see B1.5.6)
    #[instrument(skip_all)]
    fn push_stack(&mut self, excp_typ: ExceptionType) -> Result<(), super::Error> {
        // if fp_enabled && self.control.fpca() {
        //     framesize = 0x68;
        //     forcealign = true;
        // }
        let framesize = 0x20u32;
        let forcealign = self.scs.get_ccr().stkalign();

        let spmask = !(forcealign as u32);
        
        let (frameptralign, frameptr) = if self.control.spsel() && self.mode == Mode::Thread {
            let proc_sp = self.get_proc_sp()?.offset() as u32;
            let frameptralign = (proc_sp >> 2) & forcealign as u32;
            let frameptr = (proc_sp - framesize) & spmask;
            (frameptralign, Address::from(frameptr))
        } else {
            let main_sp = self.get_main_sp()?.offset() as u32;
            let frameptralign = (main_sp >> 2) & forcealign as u32;
            let frameptr = (main_sp - framesize) & spmask;
            (frameptralign, Address::from(frameptr))
        };
        // update current sp
        self.write_sp(&frameptr)
            .map_err(|_| {
                let msg = "could not update sp during stack push";
                error!(msg);
                super::Error::System(msg)
            })?;

        let t = self.lang.translator();
        let push_regs = ["r0", "r1", "r2", "r3", "r12", "lr"].into_iter()
            .map(|reg_str| {
                t.register_by_name(reg_str).unwrap()
            });
        // store registers in stack frameptr
        for (i, reg) in push_regs.enumerate() {
            let bytes = self.regs.view_bytes(reg.offset() as usize, reg.size())
                .map_err(|_| {
                    let msg = "failed to read bytes from register";
                    error!("{msg}: {reg:#x?}");
                    super::Error::System(msg)
                })?;
            self.mmap.store_bytes(&(frameptr + i * 4), bytes, &mut self.events)
                .map_err(|_| {
                    let msg = "failed to push register to stack";
                    error!("{msg}: {reg:#x?} @ {:#x?}", frameptr + i * 4);
                    super::Error::System(msg)
                })?;
        }
        // push return address
        let return_address = self.return_address(excp_typ)?;
        let return_address = u32::to_le_bytes(return_address.offset() as u32);
        self.mmap.store_bytes(&(frameptr + 0x18u64), &return_address, &mut self.events)
            .map_err(|_| {
                let msg = "failed to push return address to stack";
                error!("{msg}: {:#x?}", frameptr + 0x18u64);
                super::Error::System(msg)
            })?;
        // push xpsr
        let xpsr = self.xpsr.0 & !(1 << 9) | (frameptralign << 9);
        let xpsr = u32::to_le_bytes(xpsr);
        self.mmap.store_bytes(&(frameptr + 0x1Cu64), &xpsr, &mut self.events)
            .map_err(|_| {
                let msg = "failed to push xpsr to stack";
                error!("{msg}: {:#x?}", xpsr);
                super::Error::System(msg)
            })?;
        
        // if fp_enabled && self.control.fpca() {
        //     // load floating point registers.
        //     // unimplemented.
        //     // see reference manual for details
        // }

        // if fp_enabled {
        //     if matches!(self.mode, Mode::Handler(_)) {
        //         let lr = 0xFFFFFFE0 | (((!self.control.fpca()) as u32) << 4) | 0x1;
        //     }
        //     else {
        //         let lr = 0xFFFFFFE0 | (((!self.control.fpca()) as u32) << 4) | (((!self.control.fpca()) as u32) << 2) | 0b1001;
        //     }
        // }
        let lr = if matches!(self.mode, Mode::Handler(_)) {
            0xFFFFFFF1
        } else {
            0xFFFFFFF1 | ((self.control.spsel() as u32) << 2)
        };
        let lr = BitVec::from_u32(lr, 32);
        let lr_vnd = t.register_by_name("lr").unwrap();
        self.regs.write_val_with(lr_vnd.offset() as usize, &lr, self.endian)
            .map_err(|_| {
                let msg = "failed to write to lr";
                error!("{msg}: {lr}");
                super::Error::System(msg)
            })?;

        Ok(())
    }

    /// get return address based on exception type
    /// (see B1.5.6 B1-534)
    /// must always be halfword aligned.
    /// xpsr.it bits saved to stack are consistent with return_address
    #[instrument(skip_all)]
    pub fn return_address(&self, excp_typ: ExceptionType) -> Result<Address, super::Error> {
        let result = match excp_typ {
            ExceptionType::NMI
            | ExceptionType::SVCall
            | ExceptionType::PendSV
            | ExceptionType::SysTick
            | ExceptionType::ExternalInterrupt(_) => {
                self._next_insn_address()
            }
            ExceptionType::MemFault
            | ExceptionType::UsageFault => {
                self._this_insn_address()
            }
            ExceptionType::HardFault
            | ExceptionType::BusFault
            | ExceptionType::DebugMonitor => {
                if self._is_exception_synchronous(excp_typ) {
                    self._this_insn_address()
                } else {
                    self._next_insn_address()
                }
            }
            _ => { unreachable!("invalid exception type: {excp_typ:?}") }
        };
        result.map(|address| Address::from(address.offset() & !1))
    }

    fn _next_insn_address(&self) -> Result<Address, super::Error> {
        // can get rid of all this if we give the context access to the cache as well.
        // cache is rwlock, so it's probably fine, and there'll be overhead regardless.
        let mut lifter = self.lang.lifter();
        let tmp_irb = IRBuilderArena::with_capacity(0x100);
        let pc_address = self.read_pc()
            .map_err(|_| {
                let msg = "failed to read current pc";
                warn!("{msg}");
                super::Error::UnpredictableBehavior(msg)
            })?;
        let address = Address::from(pc_address.offset() & !1);
        let bytes = self.mmap.mem_view_bytes(&address, None)
            .map_err(|_| {
                let msg = "failed to read bytes at current pc";
                error!("{msg}: {address:#x?}");
                super::Error::System(msg)
            })?;
        let pcode = lifter.lift(&tmp_irb, pc_address, bytes)
            .map_err(|_| {
                let msg = "pc at invalid instruction";
                warn!("{msg}: {pc_address}");
                super::Error::UnpredictableBehavior(msg)
            })?;
        Ok(pc_address + pcode.len())
    }

    fn _this_insn_address(&self) -> Result<Address, super::Error> {
        self.read_pc()
            .map_err(|_| {
                let msg = "failed to read current pc";
                warn!("{msg}");
                super::Error::UnpredictableBehavior(msg)
            })
    }

    fn _is_exception_synchronous(&self, _typ: ExceptionType) -> bool {
        // this is a helper that gets used in ReturnAddress (see B1.5.6)
        // but is not definied in the reference manual.
        // it seems like hard faults, bus faults, and debug exceptions may or may not
        // be synchronous.
        // need to figure out the conditions under which this is the case.
        // - system related events (A2.4.1)
        // note that synchronous means the issue is related to the currently
        // executing instruction.
        // for now assume they're always synchronous
        true
    }

    /// take an exception (B1.5.6)
    pub(crate) fn exception_taken(&mut self, typ: ExceptionType) -> Result<(), super::Error> {
        let t = self.lang.translator();
        let clear_regs = ["r0", "r1", "r2", "r3", "r12"].into_iter()
            .map(|reg_str| {
                t.register_by_name(reg_str).unwrap()
            });
        let unknown = BitVec::from_u32(0, 32);
        for reg in clear_regs {
            self.regs.write_val_with(reg.offset() as usize, &unknown, self.endian)
                .map_err(|_| {
                    let msg = "failed to write register";
                    error!("{msg}: {}", reg.display(t));
                    super::Error::System(msg)
                })?;
        }
        let vtor = Address::from(self.scs.get_vtor().tbloff() << 7);
        let vt = self.mmap.mem_view_bytes(&vtor, None)
            .map_err(|_| {
                let msg = "failed to view vector table";
                error!("{msg}: {vtor:#x?}");
                super::Error::System(msg)
            })?;
        let offset = (u32::from(&typ) * 4) as usize;
        let target = u32::from_le_bytes(unsafe {
            *(&vt[offset..offset+4] as *const [u8] as *const [u8; 4])
        });
        self._branch_to(&Address::from(target))?;
        let tbit = (target & 1) == 1;
        self.mode = Mode::Handler(typ);
        *self.xpsr.apsr_mut() = super::system::APSR::new();
        self.xpsr.ipsr_mut().set_exception_number(u32::from(&typ));
        self.xpsr.epsr_mut().set_t(tbit);
        self.xpsr.epsr_mut().itstate().set(0);
        self.control.set_fpca(false);
        self.control.set_spsel(false);
        self.scs.set_exception_active(typ);
        self.scs.update_regs()?;
        self._clear_exclusive_local()?;
        self.event.0 = true;
        self.instruction_synchronization_barrier(0xF)
    }

    /// a private helper to set the program counter
    fn _branch_to(&mut self, target: &Address) -> Result<(), super::Error> {
        self.write_pc(target)
            .map_err(|_| {
                let msg = "failed to branch to target";
                error!("{msg}: {target:#x?}");
                super::Error::System(msg)
            })
    }

    /// clear exclusive local processor id
    /// not sure what this should be doing.
    /// see B1.5.6 ExceptionTaken
    fn _clear_exclusive_local(&mut self) -> Result<(), super::Error> {
        Ok(())
    }

    /// produces instruction synchronization barrier
    /// with 4 bit option
    /// see page D6-824
    pub(crate) fn instruction_synchronization_barrier(
        &mut self,
        _option: u8,
    ) -> Result<(), super::Error> {
        // i don't know what exactly an instruction synchronization barrier
        // should be doing...
        Ok(())
    }

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
                let Some(priority) = self.scs.get_exception_priority(*typ) else {
                    warn!("processor may be in inconsistent state: no exception registered for exception: {typ:?}");
                    return false;
                };
                
                priority < self.scs.current_priority(&self.basepri, &self.primask, &self.faultmask)
            }
            // a debug event with debug enabled
            Event::Debug(_) => {
                let offset = DebugRegType::DHCSR.offset() / 4;
                let backing: &[u32; 0x400] = self.scs.as_ref();
                let dhcsr = DHCSR::from_bits(backing[offset]);
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
                let Some(priority) = self.scs.get_exception_priority(*typ) else {
                    warn!("processor may be in inconsistent state: no exception registered for exception: {typ:?}");
                    return false;
                };
                
                let vecactive = self.scs.get_icsr().vectactive();
                let current_typ: ExceptionType = vecactive.into();
                let Some(unmasked_current_priority) = self.scs.get_exception_priority(current_typ) else {
                    panic!("processor is in inconsistent state: no exception registered for current exception: {current_typ:?}");
                };

                priority < unmasked_current_priority
            }
            // a debug event with debug enabled
            Event::Debug(_) => {
                let offset = DebugRegType::DHCSR.offset() / 4;
                let backing: &[u32; 0x400] = self.scs.as_ref();
                let dhcsr = DHCSR::from_bits(backing[offset]);
                dhcsr.s_halt()
            }
            // other implementation-defined events

            _ => { false }
        }
    }
}