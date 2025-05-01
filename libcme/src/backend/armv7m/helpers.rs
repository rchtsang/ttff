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

    pub fn set_main_sp(&mut self, address: &Address) -> Result<(), super::Error> {
        if self.is_sp_main() {
            self.write_sp(address)
                .map_err(|_| super::Error::System("failed to write sp"))
        } else {
            self.main_sp = Some(address.offset() as u32);
            Ok(())
        }
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

    pub fn set_proc_sp(&mut self, address: &Address) -> Result<(), super::Error> {
        if !self.is_sp_main() {
            self.write_sp(address)
                .map_err(|_| super::Error::System("failed to write sp"))
        } else {
            self.proc_sp = Some(address.offset() as u32);
            Ok(())
        }
    }

    /// current exception priority
    /// from B1.5.4 page B1-529
    #[allow(unused)]
    pub fn current_priority(&self) -> i16 {
        // priority of thread mode with no active exceptions
        // this value is PriorityMax + 1 = 256
        // (configurable priority maximum bit field is 8 bits)
        let mut highestpri: i16 = 256;
        // priority influence of basepri, primask, and faultmask
        let mut boostedpri: i16 = 256;

        let subgroupshift = self.scs.get_aircr().prigroup();
        // used by priority grouping
        let groupvalue  = 0b10 << subgroupshift;

        // valid ipsr values should be in range of 2 to 511
        // to save time, we keep a list of active exceptions
        // instead of looping over the full range of exception values.
        // if desired, we can switch to looping to save memory and
        // removing nvic.active list
        for excp_type in self.scs.exceptions.active() {
            let excp_num = u32::from(excp_type) as u8;
            let pri = self.scs.nvic_regs()
                .get_ipr(excp_num / 4)
                .pri_n(excp_num % 4);
            if (pri as i16) < highestpri {
                highestpri = pri as i16;

                // include prigroup effect
                highestpri -= highestpri % groupvalue;
            }
        }

        if self.basepri.basepri() != 0 {
            boostedpri = self.basepri.basepri() as i16;

            // include prigroup effect
            boostedpri -= boostedpri % groupvalue;
        }

        if self.primask.pm() {
            boostedpri = 0;
        }

        if self.faultmask.fm() {
            boostedpri = -1;
        }

        if boostedpri < highestpri {
            boostedpri
        } else {
            highestpri
        }
    }

    /// exception entry (see B1.5.6)
    #[instrument(skip_all)]
    pub fn exception_entry(&mut self, excp_typ: ExceptionType) -> Result<ThreadSwitch, super::Error> {
        let old_thread = self.current_thread();
        let switch_address = self.read_pc()
            .map_err(|_| {
                let msg = "could not read pc during exception entry";
                error!(msg);
                super::Error::System(msg)
            })?;
        let old_frame_address = if self.control.spsel() && self.mode == Mode::Thread {
            self.get_proc_sp()?
        } else {
            self.get_main_sp()?
        };
        let (return_address, new_frame_address) = self.push_stack(excp_typ)?;
        let vtor = Some(Address::from(self.scs.get_vtor().tbloff() << 7));
        let target_address = self.exception_taken(excp_typ)?;
        let new_thread = self.current_thread();
        let return_address = Some(return_address);
        let typ = u32::from(&excp_typ);

        Ok(ThreadSwitch {
            typ,
            old_thread,
            new_thread,
            old_frame_address,
            new_frame_address,
            switch_address,
            target_address,
            return_address,
            vtor,
        })
    }

    /// push stack variables on exception entry (see B1.5.6)
    /// returns the return address and frame address
    #[instrument(skip_all)]
    fn push_stack(&mut self, excp_typ: ExceptionType) -> Result<(Address, Address), super::Error> {
        // if fp_enabled && self.control.fpca() {
        //     framesize = 0x68;
        //     forcealign = true;
        // }
        let framesize = 0x20u32;
        let forcealign = self.scs.get_ccr().stkalign();

        let spmask = !((forcealign as u32) << 2);
        
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
        let result = return_address.clone();
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

        Ok((result, frameptr))
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
    /// return the target address
    pub(crate) fn exception_taken(&mut self, typ: ExceptionType) -> Result<Address, super::Error> {
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
        let tbit = (target & 1) == 1;
        let target_address = Address::from(target);
        self._branch_to(&target_address)?;
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
        self.instruction_synchronization_barrier(0xF)?;
        Ok(target_address)
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

    /// perform exception return
    /// returns context switch information
    #[instrument(skip_all)]
    pub fn exception_return(&mut self, exc_return: EXC_RETURN) -> Result<ThreadSwitch, super::Error> {
        assert_eq!(exc_return.exc_value(), 0xF, "invalid EXC_RETURN");
        assert!(matches!(self.mode, Mode::Handler(_)),
            "must be in handler mode to return from exception");
        if exc_return.sbop() != 0x7FFFFF {
            let msg = "unexpected SBOP reserved field value";
            error!("{msg}: {:#x}", exc_return.into_bits());
            return Err(super::Error::UnpredictableBehavior(msg));
        }
        // if fp_enabled {
        //     assert_eq!(exc_return.nofpext(), false, "unpredictable behavior")
        // }

        let returning_excp = ExceptionType::from(self.xpsr.ipsr().exception_number());
        // used for Handler -> Thread check when value == 1
        let nested_activation = self.scs.exceptions.active().len();

        // returning from inactive handler will trigger a usagefault
        // that will preempt instead of returning
        if !self.scs.exceptions.active().contains(&returning_excp) {
            warn!("returning from inactive handler is a usagefault");
            let ufsr = self.scs.get_cfsr().usagefault()
                .with_invpc(true);
            return self._return_usagefault(returning_excp, exc_return, ufsr)
        }

        let frameptr = match exc_return.modebits() {
            0b0001 => {
                // return to previous exception handler
                self.control.set_spsel(false);
                self.get_main_sp()?
            }
            0b1001 if nested_activation == 1 || self.scs.get_ccr().nonbasethrdena() => {
                // return to thread with main stack
                self.control.set_spsel(false);
                self.get_main_sp()?
            }
            0b1101 if nested_activation == 1 || self.scs.get_ccr().nonbasethrdena() => {
                // return to thread with process stack
                self.control.set_spsel(true);
                self.get_proc_sp()?
            }
            _ => {
                // return to thread exception mismatch
                // or illegal exc_return
                let ufsr = self.scs.get_cfsr().usagefault()
                    .with_invpc(true);
                return self._return_usagefault(returning_excp, exc_return, ufsr)
            }
        };

        self._deactivate_exception(returning_excp);
        let old_thread = self.current_thread();
        let switch_address = self.read_pc()
            .map_err(|_| {
                let msg = "failed to read pc during exception return";
                error!(msg);
                super::Error::System(msg)
            })?;
        let old_frame_address = frameptr.clone();
        let (new_frame_address, target_address) = self.pop_stack(&frameptr, exc_return)?;
        self.mode = if exc_return.modebits() == 0b0001 {
            // set mode for returning to handler
            let excp_num = self.xpsr.ipsr().exception_number();
            if excp_num == 0 {
                // return ipsr is inconsistent
                let ufsr = self.scs.get_cfsr().usagefault()
                    .with_invpc(true);
                // push stack again to negate popstack
                self.push_stack(ExceptionType::UsageFault)?;
                return self._return_usagefault(returning_excp, exc_return, ufsr);
            }
            let typ = ExceptionType::from(excp_num);
            Mode::Handler(typ)
        } else {
            // set mode for returning to thread
            let excp_num = self.xpsr.ipsr().exception_number();
            if excp_num != 0 {
                // return ipsr is inconsistent
                let ufsr = self.scs.get_cfsr().usagefault()
                    .with_invpc(true);
                // push stack again to negate popstack
                self.push_stack(ExceptionType::UsageFault)?;
                return self._return_usagefault(returning_excp, exc_return, ufsr);
            }
            Mode::Thread
        };
        self._branch_to(&target_address)?;
        let return_address = None;
        let new_thread = self.current_thread();

        self._clear_exclusive_local()?;
        self.event.0 = true;
        self.instruction_synchronization_barrier(0xF)?;

        if self.mode == Mode::Thread 
            && nested_activation == 0 
            && self.scs.get_scr().sleeponexit()
        {
            return self.sleep_on_exit();
        }

        Ok(ThreadSwitch {
            typ: 0,
            vtor: None,
            old_thread,
            new_thread,
            old_frame_address,
            new_frame_address,
            switch_address,
            target_address,
            return_address,
        })
    }

    #[instrument(skip_all)]
    fn _deactivate_exception(
        &mut self,
        typ: ExceptionType,
    ) {
        self.scs.clr_exception_active(typ);
        // PRIMASK and BASEPRI unchanged on exception exit
        if self.xpsr.ipsr().exception_number() != 2 {
            // clear faultmask on any return except nmi
            self.faultmask.set_fm(false);
        }
    }

    fn _return_usagefault(
        &mut self,
        returning_excp: ExceptionType,
        exc_return: EXC_RETURN,
        ufsr: UFSR,
    ) -> Result<ThreadSwitch, super::Error> {
        // get current context information
        let old_thread = self.current_thread();
        let switch_address = self.read_pc()
            .map_err(|_| {
                let msg = concat!(
                    "could not read pc while triggering usagefault ",
                    "during exception return",
                );
                error!(msg);
                super::Error::System(msg)
            })?;

        // get frame address, return address, and xpsr pushed when exception was entered
        let pushed_frame_address = if exc_return.modebits() == 0xD {
            // use process return stack
            self.get_proc_sp()?
        } else { self.get_main_sp()? };
        let pushed_return_address = self.mmap
            .mem_view_bytes(&(pushed_frame_address + 0x18u64), Some(4))
            .map(|slice| unsafe {
                u32::from_le_bytes(*(&slice[..4] as *const [u8] as *const[u8; 4]))
            }).map_err(|_| {
                let msg = concat!(
                    "failed to read stack frame while triggering",
                    "usagefault during exception return",
                );
                error!("{msg}: {pushed_frame_address:#x?}");
                super::Error::System(msg)
            })?;
        let pushed_return_address = Address::from(pushed_return_address);

        self._deactivate_exception(returning_excp);
        self.scs.get_cfsr_mut().set_usagefault(ufsr);
        let value = BitVec::from_u32(exc_return.into_bits(), 32);
        let lr_vnd = self.lang.translator().register_by_name("lr").unwrap();
        self.regs.write_val_with(lr_vnd.offset() as usize, &value, self.endian)
            .map_err(|_| {
                let msg = concat!(
                    "failed to write lr while triggering usagefault ",
                    "during exception return",
                );
                error!("{msg}: {value:#x}");
                super::Error::System(msg)
            })?;
        
        let vtor = Some(Address::from(self.scs.get_vtor().tbloff() << 7));
        let target_address = self.exception_taken(ExceptionType::UsageFault)?;
        let new_thread = self.current_thread();
        let old_frame_address = pushed_frame_address;
        let new_frame_address = pushed_frame_address;
        let return_address = Some(pushed_return_address);
        return Ok(ThreadSwitch {
            typ: u32::from(&ExceptionType::UsageFault),
            old_thread,
            new_thread,
            old_frame_address,
            new_frame_address,
            switch_address,
            target_address,
            return_address,
            vtor,
        })
    }

    /// pop the stack, returning the new frame address and the target pc address
    #[instrument(skip_all)]
    fn pop_stack(&mut self, frameptr: &Address, exc_return: EXC_RETURN) -> Result<(Address, Address), super::Error> {
        // if fp_enabled && !exc_return.nofpext() {
        //     framesize = 0x68;
        //     forcealign = true;
        // }
        let framesize = 0x20u32;
        let forcealign = self.scs.get_ccr().stkalign() as u32;

        let t = self.lang.translator();
        let pop_regs = ["r0", "r1", "r2", "r3", "r12", "lr"].into_iter()
            .map(|reg_str| { t.register_by_name(reg_str).unwrap() });
        for (i, reg) in pop_regs.enumerate() {
            let bytes = self.mmap.mem_view_bytes(&(*frameptr + i as u64 * 4), Some(4))
                .map_err(|_| {
                    let msg = "failed to read from stack frame";
                    error!("{msg}: {frameptr:#x?}");
                    super::Error::System(msg)
                })?;
            self.regs.write_bytes(reg.offset() as usize, bytes)
                .map_err(|_| {
                    let msg = "failed to write to register";
                    error!("{msg}: {}", reg.display(t));
                    super::Error::System(msg)
                })?;
        }
        let (target_address, psr) = self.mmap
            .mem_view_bytes(&(*frameptr + 0x18u64), Some(8))
            .map_err(|_| {
                let msg = "failed to read from stack frame";
                error!("{msg}: {frameptr:#x?}");
                super::Error::System(msg)
            }).map(|slice| unsafe {(
                u32::from_le_bytes(*(&slice[..4] as *const [u8] as *const [u8; 4])),
                u32::from_le_bytes(*(&slice[4..] as *const [u8] as *const [u8; 4])),
            )})?;

        // if fp_enabled {
        //     // see PopStack in  B1.5.8 for these implementation details
        // }

        let spmask = (((psr >> 9) & 1) & forcealign) << 2;

        let frame_address = match exc_return.modebits() {
            0b0001
            | 0b1001 => {
                // returning to handler
                let main_sp = self.get_main_sp()?.offset() as u32;
                let frame_address = Address::from((main_sp + framesize) | spmask);
                self.set_main_sp(&frame_address)?;
                frame_address
            }
            0b1101 => {
                let proc_sp = self.get_proc_sp()?.offset() as u32;
                let frame_address = Address::from((proc_sp + framesize) | spmask);
                self.set_proc_sp(&frame_address)?;
                frame_address
            }
            _ => { panic!("invalid EXC_RETURN: {:#x}", exc_return.modebits()) }
        };

        self.xpsr.0 = psr;
        let target_address = Address::from(target_address);
        Ok((frame_address, target_address))
    }

    fn sleep_on_exit(&mut self) -> Result<ThreadSwitch, super::Error> {
        unimplemented!("sleep on exit implementation defined")
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
                let priority = self.scs.get_exception_priority(*typ);
                
                priority < self.current_priority()
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
                let priority = self.scs.get_exception_priority(*typ);
                let vecactive = self.scs.get_icsr().vectactive();
                let current_typ: ExceptionType = vecactive.into();
                let unmasked_current_priority = self.scs.get_exception_priority(current_typ);

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