//! helpers.rs
//! 
//! separating system control space helper functions

use super::*;

impl SysCtrlSpace {
    /// returns the current group and subgroup priority without boosting
    pub fn get_exception_priority(
        &self,
        typ: ExceptionType,
    ) -> (i16, i16) {
        let excp_num = u32::from(&typ);
        let pri = match excp_num {
             1 => { -3 }
             2 => { -2 }
             3 => { -1 }
             4 => { self.get_shpr1().pri_4() as i16 }
             5 => { self.get_shpr1().pri_5() as i16 }
             6 => { self.get_shpr1().pri_6() as i16 }
             7 => { self.get_shpr1().pri_7() as i16 }
             8 => { self.get_shpr2().pri_8() as i16 }
             9 => { self.get_shpr2().pri_9() as i16 }
            10 => { self.get_shpr2().pri_10() as i16 }
            11 => { self.get_shpr2().pri_11() as i16 }
            12 => { self.get_shpr3().pri_12() as i16 }
            13 => { self.get_shpr3().pri_13() as i16 }
            14 => { self.get_shpr3().pri_14() as i16 }
            15 => { self.get_shpr3().pri_15() as i16 }
            16..=511 => {
                let int_num = excp_num - 16;
                let n = (int_num / 32) as u8;
                let i = (int_num % 32) as u8;
                self.nvic_regs().get_ipr(n).pri_n(i) as i16
            }
            _ => { panic!("invalid exception number: {excp_num}") }
        };
        if pri < 0 {
            (pri, 0)
        } else {
            let subgroupshift = self.get_aircr().prigroup() as i16;
            let groupvalue = 0b10_i16 << subgroupshift;
            let subgroup_pri = pri % groupvalue;
            let group_pri = pri - subgroup_pri;
            (group_pri, subgroup_pri)
        }
    }

    pub fn set_exception_priority(
        &mut self,
        typ: ExceptionType,
        pri: u8,
    ) {
        let excp_num = u32::from(&typ);
        match excp_num {
            1 => { panic!("cannot change {typ:?} priority") }
            2 => { panic!("cannot change {typ:?} priority") }
            3 => { panic!("cannot change {typ:?} priority") }
            4 => { self.get_shpr1_mut().set_pri_4(pri) }
            5 => { self.get_shpr1_mut().set_pri_5(pri) }
            6 => { self.get_shpr1_mut().set_pri_6(pri) }
            7 => { self.get_shpr1_mut().set_pri_7(pri) }
            8 => { self.get_shpr2_mut().set_pri_8(pri) }
            9 => { self.get_shpr2_mut().set_pri_9(pri) }
           10 => { self.get_shpr2_mut().set_pri_10(pri) }
           11 => { self.get_shpr2_mut().set_pri_11(pri) }
           12 => { self.get_shpr3_mut().set_pri_12(pri) }
           13 => { self.get_shpr3_mut().set_pri_13(pri) }
           14 => { self.get_shpr3_mut().set_pri_14(pri) }
           15 => { self.get_shpr3_mut().set_pri_15(pri) }
           16..=511 => {
               let int_num = excp_num - 16;
               let n = (int_num / 32) as u8;
               let i = (int_num % 32) as u8;
               self.nvic_regs_mut().get_ipr_mut(n).set_pri_n(i, pri)
           }
           _ => { panic!("invalid exception number: {excp_num}") }
        }
        self.sort_pending();
    }
}


impl SysCtrlSpace {
    /// sort pending exceptions
    fn sort_pending(&mut self) {
        let mut pending = vec![];
        for typ in self.exceptions.pending.iter() {
            pending.push((self.get_exception_priority(*typ), *typ));
        }
        // would need to implement the sorting algorithm manually to do in-place
        // and that would be potentially a lot more comparisons that invoke 
        // get_exception_priority, which is expensive-ish
        pending.sort_by(|v1, v2| v1.0.cmp(&v2.0));
        self.exceptions.pending = pending.into_iter()
            .map(|(_, typ)| typ)
            .collect();
    }

    /// update all exception registers based on current exceptions state
    #[instrument(skip_all)]
    pub fn update_exception_regs(&mut self) {
        // clear all exception enable/pending/active bits/registers
        // enabled
        self.get_shcsr_mut().set_usgfaultena(false);
        self.get_shcsr_mut().set_busfaultena(false);
        self.get_shcsr_mut().set_memfaultena(false);
        // pending
        self.get_icsr_mut().set_vectpending(0);
        self.get_icsr_mut().set_pendstset(false);
        self.get_icsr_mut().set_pendsvset(false);
        self.get_icsr_mut().set_isrpending(false);
        self.get_shcsr_mut().set_svcallpended(false);
        self.get_shcsr_mut().set_busfaultpended(false);
        self.get_shcsr_mut().set_memfaultpended(false);
        self.get_shcsr_mut().set_usgfaultpended(false);
        // active
        self.get_icsr_mut().set_vectactive(0);
        self.get_icsr_mut().set_rettobase(true);
        self.get_icsr_mut().set_isrpreempt(false); // irrelevant while debug mode not implemented
        self.get_icsr_mut().set_nmipendset(false);
        self.get_shcsr_mut().set_systickact(false);
        self.get_shcsr_mut().set_pendsvact(false);
        self.get_shcsr_mut().set_monitoract(false);
        self.get_shcsr_mut().set_svcallact(false);
        self.get_shcsr_mut().set_usgfaultact(false);
        self.get_shcsr_mut().set_busfaultact(false);
        self.get_shcsr_mut().set_memfaultact(false);

        let mut nvic_regs = self.nvic_regs_mut();
        for n in 0..=15 {
            // enabled
            nvic_regs.get_iser_mut(n).set_setena(0);
            nvic_regs.get_icer_mut(n).set_clrena(0);
            // pending
            nvic_regs.get_ispr_mut(n).set_setpend(0);
            nvic_regs.get_icpr_mut(n).set_clrpend(0);
            // active
            nvic_regs.get_iabr_mut(n).set_active(0);
        }

        // set enabled values
        let enabled = self.exceptions.enabled.clone();
        for typ in enabled.iter() {
            match typ {
                ExceptionType::UsageFault => {
                    self.get_shcsr_mut().set_usgfaultena(true);
                }
                ExceptionType::BusFault => {
                    self.get_shcsr_mut().set_busfaultena(true);
                }
                ExceptionType::MemFault => {
                    self.get_shcsr_mut().set_memfaultena(true);
                }
                ExceptionType::ExternalInterrupt(int_n) => {
                    let n = (int_n / 32) as u8;
                    let i = int_n % 32;
                    let mut nvic_regs = self.nvic_regs_mut();
                    let iser = nvic_regs.get_iser_mut(n);
                    let val = iser.into_bits() | (1 << i);
                    iser.set_setena(val);
                    let icer = nvic_regs.get_icer_mut(n);
                    icer.set_clrena(val);
                }
                _ => { debug!("no enabled exception registers set for {typ:?}") }
            }
        }

        // set pending values
        let pending = self.exceptions.pending.clone();
        for (i, typ) in pending.iter().enumerate() {
            let excp_num = u32::from(typ);
            if i == 0 {
                self.get_icsr_mut().set_vectpending(excp_num);
            }
            match typ {
                ExceptionType::SVCall => {
                    self.get_shcsr_mut().set_svcallpended(true);
                }
                ExceptionType::UsageFault => {
                    self.get_shcsr_mut().set_usgfaultpended(true);
                }
                ExceptionType::BusFault => {
                    self.get_shcsr_mut().set_busfaultpended(true);
                }
                ExceptionType::MemFault => {
                    self.get_shcsr_mut().set_memfaultpended(true);
                }
                ExceptionType::PendSV => {
                    self.get_icsr_mut().set_pendsvset(true);
                }
                ExceptionType::SysTick => {
                    self.get_icsr_mut().set_pendstset(true);
                }
                ExceptionType::ExternalInterrupt(int_n) => {
                    self.get_icsr_mut().set_isrpending(true);
                    
                    let n = (int_n / 32) as u8;
                    let i = int_n % 32;
                    let mut nvic_regs = self.nvic_regs_mut();
                    let ispr = nvic_regs.get_ispr_mut(n);
                    let val = ispr.into_bits() | (1 << i);
                    ispr.set_setpend(val);
                    let icpr = nvic_regs.get_icpr_mut(n);
                    icpr.set_clrpend(val);
                }
                _ => { debug!("no pending exception registers set for {typ:?}") }
            }
        }

        // set active values
        if self.exceptions.active().len() < 1 {
            self.get_icsr_mut().set_rettobase(true);
        }
        // clone needed to satisfy borrow checker.
        // to make this unnecessary, refactor SCS to have separate register space
        // so that register access and exception state access are separable
        let active = self.exceptions.active.clone();
        for (i, typ) in active.iter().enumerate() {
            let excp_num = u32::from(typ);
            if i == 0 {
                self.get_icsr_mut().set_vectactive(excp_num);
            }
            match typ {
                ExceptionType::NMI => {
                    self.get_icsr_mut().set_nmipendset(true);
                }
                ExceptionType::SVCall => {
                    self.get_shcsr_mut().set_svcallact(true);
                }
                ExceptionType::UsageFault => {
                    self.get_shcsr_mut().set_usgfaultact(true);
                }
                ExceptionType::BusFault => {
                    self.get_shcsr_mut().set_busfaultact(true);
                }
                ExceptionType::MemFault => {
                    self.get_shcsr_mut().set_memfaultact(true);
                }
                ExceptionType::PendSV => {
                    self.get_shcsr_mut().set_pendsvact(true)
                }
                ExceptionType::SysTick => {
                    self.get_shcsr_mut().set_systickact(true);
                }
                ExceptionType::DebugMonitor => {
                    self.get_shcsr_mut().set_monitoract(true);
                }
                ExceptionType::ExternalInterrupt(int_n) => {
                    self.get_icsr_mut().set_isrpending(true);
                    
                    let n = (int_n / 32) as u8;
                    let i = int_n % 32;
                    let mut nvic_regs = self.nvic_regs_mut();
                    let ispr = nvic_regs.get_ispr_mut(n);
                    let val = ispr.into_bits() | (1 << i);
                    ispr.set_setpend(val);
                    let icpr = nvic_regs.get_icpr_mut(n);
                    icpr.set_clrpend(val);
                }
                _ => { debug!("no active exception registers set for {typ:?}") }
            }
        }
    }

    #[instrument(skip_all)]
    pub fn set_exception_pending(&mut self, typ: ExceptionType) {
        info!("set pending: {typ:?}");
        self.exceptions.pending.push(typ);
        self.sort_pending();
        self.update_exception_regs();
    }

    #[instrument(skip_all)]
    pub fn clr_exception_pending(&mut self, typ: ExceptionType) {
        info!("clr pending: {typ:?}");
        let idx = self.exceptions.pending.iter()
            .position(|t| *t == typ);
        if let Some(idx) = idx {
            self.exceptions.pending.remove(idx);
            self.update_exception_regs();
        }
    }

    #[instrument(skip_all)]
    pub fn enable_exception(&mut self, typ: ExceptionType) {
        info!("enabling {typ:?}");
        if !self.exceptions.enabled.contains(&typ) {
            self.exceptions.enabled.push(typ);
            self.update_exception_regs();
        }
    }

    #[instrument(skip_all)]
    pub fn disable_exception(&mut self, typ: ExceptionType) {
        info!("disabling {typ:?}");
        let idx = self.exceptions.enabled.iter()
            .position(|t| *t == typ);
        if let Some(idx) = idx {
            self.exceptions.enabled.remove(idx);
            self.update_exception_regs();
        }
    }

    #[instrument(skip_all)]
    pub fn exception_enabled(&mut self, typ: ExceptionType) ->  bool {
        match typ {
            ExceptionType::Reset
            | ExceptionType::NMI
            | ExceptionType::HardFault
            | ExceptionType::SVCall
            | ExceptionType::PendSV
            | ExceptionType::SysTick => {
                // these exceptions are always enabled. see B1.5.1
                true
            }
            _ => { self.exceptions.enabled.contains(&typ) }
        }
    }

    #[instrument(skip_all)]
    pub fn set_exception_active(&mut self, typ: ExceptionType) {
        info!("set active: {typ:?}");
        if !self.exception_enabled(typ) {
            warn!("{typ:?} not enabled!");
            return;
        }
        if !self.exceptions.pending.contains(&typ) {
            warn!("setting non-pending exception as active: {typ:?}")
        } else {
            // move exception status from pending to active
            let idx = self.exceptions.pending.iter()
                .position(|t| *t == typ).unwrap();
            self.exceptions.pending.remove(idx);
        }
        // treat active list as a stack, always put most recently
        // activated exception at the front.
        self.exceptions.active.insert(0, typ);
        self.update_exception_regs();
    }

    #[instrument(skip_all)]
    pub fn clr_exception_active(&mut self, typ: ExceptionType) {
        info!("clr active: {typ:?}");
        if !self.exceptions.active.contains(&typ) {
            warn!("{typ:?} not active!");
            // should cause usagefault
            return;
        }
        let idx = self.exceptions.active.iter()
            .position(|t| *t == typ).unwrap();
        self.exceptions.active.remove(idx);
        self.update_exception_regs();
    }
}