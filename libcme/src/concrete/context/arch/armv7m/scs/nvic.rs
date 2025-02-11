//! nvic.rs
//! 
//! implementation of the nested vector interrupt controller for armv7m
use bitfield_struct::bitfield;
use derive_more::derive::{From, TryFrom, TryInto};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NVICRegType {
    ISER(u8),   // n = [0, 15]
    ICER(u8),   // n = [0, 15]
    ISPR(u8),   // n = [0, 15]
    ICPR(u8),   // n = [0, 15]
    IABR(u8),   // n = [0, 15]
    IPR(u8),    // n = [0, 123]
}

impl NVICRegType {
    /// lookup register corresponding to given byte offset
    pub fn lookup_offset(offset: usize) -> Option<Self> {
        assert!(((offset >= 0x100) && (offset <= 0xd00)), "offset not in nvic!");
        match offset {
            0x100..=0x13c => { Some(NVICRegType::ISER(((offset - 0x100) / 4) as u8)) }
            0x180..=0x1bc => { Some(NVICRegType::ICER(((offset - 0x180) / 4) as u8)) }
            0x200..=0x23c => { Some(NVICRegType::ISPR(((offset - 0x200) / 4) as u8)) }
            0x280..=0x2bc => { Some(NVICRegType::ICPR(((offset - 0x280) / 4) as u8)) }
            0x300..=0x33c => { Some(NVICRegType::IABR(((offset - 0x300) / 4) as u8)) }
            0x400..=0x5ec => { Some(NVICRegType::IPR(((offset - 0x400) / 4) as u8)) }
            _ => { None }
        }
    }
}

impl NVICRegType {
    pub fn offset(&self) -> usize {
        match self {
            NVICRegType::ISER(n) => { 0x100 + (4 * *n as usize) }
            NVICRegType::ICER(n) => { 0x180 + (4 * *n as usize) }
            NVICRegType::ISPR(n) => { 0x200 + (4 * *n as usize) }
            NVICRegType::ICPR(n) => { 0x280 + (4 * *n as usize) }
            NVICRegType::IABR(n) => { 0x300 + (4 * *n as usize) }
            NVICRegType::IPR(n)  => { 0x400 + (4 * *n as usize) }
        }
    }

    pub fn permissions(&self) -> u8 {
        match self {
            NVICRegType::IABR(_) => { 0b100 }
            _ => { 0b110 }
        }
    }

    pub fn reset(&self) -> Option<u32> {
        // all have reset value 0
        Some(0)
    }
}

#[derive(Debug)]
pub struct NVICRegs<'a> {
    pub backing: &'a mut [u32; 0x340]
}

impl<'a> NVICRegs<'a> {
    pub fn new(backing: &'a mut [u32; 0x340]) -> Self {
        Self { backing }
    }

    /// perform an event-triggering read of nvic register bytes
    pub fn read_bytes(&mut self,
        offset: usize,
        dst: &mut [u8],
        _events: &mut VecDeque<Event>,
    ) -> Result<(), context::Error> {
        let word_offset = offset / 4;
        let address = BASE + offset as u32;
        let reg_type = NVICRegType::lookup_offset(offset)
            .ok_or_else(| | {
                ArchError::from(Error::InvalidSysCtrlReg(address.into()))
            })?;
        match reg_type {
            NVICRegType::IPR(_n) => {
                check_alignment(address, dst.len(), Alignment::Any)?;
            }
            _ => {
                check_alignment(address, dst.len(), Alignment::Word)?;
            }
        }
        let reg_slice = self._view_bytes(word_offset);
        let byte_offset = offset & 0b11;
        dst.copy_from_slice(&reg_slice[byte_offset..]);
        Ok(())
    }

    /// perform an event-triggering write of nvic register bytes
    pub fn write_bytes(&mut self,
        offset: usize,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), context::Error> {
        check_alignment(BASE + offset as u32, src.len(), Alignment::Any)?;
        let word_offset = offset / 4;
        let reg_type = NVICRegType::lookup_offset(offset)
            .ok_or_else( | | {
                let address = Address::from(BASE + offset as u32);
                ArchError::from(Error::InvalidSysCtrlReg(address))
            })?;
        let write_val = src.iter().enumerate().take(4)
            .fold(0u32, |val, (i, &byte)| {
                val | ((byte as u32) << i)
            });
        match reg_type {
            NVICRegType::IPR(_n) => {
                let byte_offset = offset & 0b11;
                let slice = self._view_bytes_mut(word_offset);
                let slice = &mut slice[byte_offset..];
                slice.copy_from_slice(src);
            }
            NVICRegType::ISER(n) => {
                check_alignment(BASE + offset as u32, src.len(), Alignment::Word)?;
                // upper bits ignored for n == 15
                let write_val = if n == 15 { write_val & 0xFFFF } else { write_val };
                
                let iser = self.get_iser_mut(n);
                let masked_set_val  = (iser.0 ^ write_val) & write_val;
                for bit_n in BitIter::from(masked_set_val) {
                    let ext_num = (32 * n as u32) + bit_n as u32;
                    let excp = ExceptionType::from(16 + ext_num);
                    events.push_back(Event::ExceptionEnabled(excp, true));
                }
                iser.0 |= masked_set_val;

                // update icer by setting changed bits (keep enable status consistent)
                let icer = self.get_icer_mut(n);
                icer.0 |= masked_set_val;
            }
            NVICRegType::ICER(n) => {
                check_alignment(BASE + offset as u32, src.len(), Alignment::Word)?;
                // upper bits ignored for n == 15
                let write_val = if n == 15 { write_val & 0xFFFF } else { write_val };
                
                let icer = self.get_icer_mut(n);
                let masked_set_val  = (icer.0 ^ write_val) & write_val;
                for bit_n in BitIter::from(masked_set_val) {
                    let ext_num = (32 * n as u32) + bit_n as u32;
                    let excp = ExceptionType::from(16 + ext_num);
                    events.push_back(Event::ExceptionEnabled(excp, false));
                }
                icer.0 &= !masked_set_val;

                // update iser by clearing changed bits (keep enable status consistent)
                let iser = self.get_iser_mut(n);
                iser.0 &= !masked_set_val;

            }
            NVICRegType::ISPR(n) => {
                check_alignment(BASE + offset as u32, src.len(), Alignment::Word)?;
                // upper bits ignored for n == 15
                let write_val = if n == 15 { write_val & 0xFFFF } else { write_val };
                
                let ispr  = self.get_ispr_mut(n);
                let masked_set_val  = (ispr.0 ^ write_val) & write_val;
                for bit_n in BitIter::from(masked_set_val) {
                    let ext_num = (32 * n as u32) + bit_n as u32;
                    let excp = ExceptionType::from(16 + ext_num);
                    events.push_back(Event::ExceptionSetPending(excp, true));
                }
                ispr.0 |= masked_set_val;

                // update icpr by setting changed bits
                let icpr = self.get_icpr_mut(n);
                icpr.0 |= masked_set_val;
            }
            NVICRegType::ICPR(n) => {
                check_alignment(BASE + offset as u32, src.len(), Alignment::Word)?;
                // upper bits ignored for n == 15
                let write_val = if n == 15 { write_val & 0xFFFF } else { write_val };
                
                let icpr = self.get_icpr_mut(n);
                let masked_set_val = (icpr.0 ^ write_val) & write_val;
                for bit_n in BitIter::from(masked_set_val) {
                    let ext_num = (32 * n as u32) + bit_n as u32;
                    let excp = ExceptionType::from(16 + ext_num);
                    events.push_back(Event::ExceptionSetPending(excp, false));
                }
                icpr.0 &= !masked_set_val;

                // update ispr by clearing changed bits
                let ispr = self.get_ispr_mut(n);
                ispr.0 &= !masked_set_val;
            }
            NVICRegType::IABR(_n) => {
                // iabr is read-only
                let address: Address = (BASE + offset as u32).into();
                let err = Error::WriteAccessViolation(address);
                return Err(ArchError::from(err).into());
            }
        }
        Ok(())
    }
}

#[allow(unused)]
/// state for nested vector interrupt controller
#[derive(Debug, Clone)]
pub struct NVICState {
    pub(crate) vtsize: usize,

    pub(crate) internal: [Exception; 16],
    pub(crate) external: Vec<Exception>,
    pub(crate) queue: Vec<ExceptionType>,
}

impl Default for NVICState {
    fn default() -> Self {
        let vtsize = 16;
        let internal = [
            Exception::default(),
            Exception::new_with(ExceptionType::Reset,         -3, None),
            Exception::new_with(ExceptionType::NMI,           -2, None),
            Exception::new_with(ExceptionType::HardFault,     -1, None),
            Exception::new_with(ExceptionType::MemFault,       0, None),
            Exception::new_with(ExceptionType::BusFault,       0, None),
            Exception::new_with(ExceptionType::UsageFault,     0, None),
            Exception::new_with(ExceptionType::Reserved(7),    0, None),
            Exception::new_with(ExceptionType::Reserved(8),    0, None),
            Exception::new_with(ExceptionType::Reserved(9),    0, None),
            Exception::new_with(ExceptionType::Reserved(10),   0, None),
            Exception::new_with(ExceptionType::SVCall,         0, None),
            Exception::new_with(ExceptionType::DebugMonitor,   0, None),
            Exception::new_with(ExceptionType::Reserved(13),   0, None),
            Exception::new_with(ExceptionType::PendSV,         0, None),
            Exception::new_with(ExceptionType::SysTick,        0, None),
        ];
        let external = vec![];
        let queue = vec![];
        Self { vtsize, internal, external, queue }
    }
}

impl NVICState {
    pub fn new_with(vt: &[u8]) -> Self {
        let mut state = Self::default();
        state.update(vt);
        state
    }

    /// update vectors in the saved nvic state from a vector table.
    /// note that this does _not_ update the exception queue.
    pub fn update(&mut self, vt: &[u8]) -> &mut Self {
        assert!(vt.len() >= 16 * 4, "vector table must have arch-defined exceptions");
        for (i, exception) in self.internal.iter_mut()
            .skip(1).enumerate()
        {
            let address = unsafe {
                let offset = i * 4;
                *(&vt[offset..] as *const [u8] as *const [u8; 4] as *const u32)
            };
            exception.entry = Some(address.into())
        }
        self.external.clear();
        for (i, entry) in vt.chunks(4)
            .skip(16).enumerate()
        {
            let typ = ExceptionType::ExternalInterrupt(16 + i as u32);
            let excp = Exception::new_with(typ, 0, Some(entry));
            self.external.push(excp);
        }
        self
    }

    /// get current state of given exception type
    pub fn get_exception(&self, typ: &ExceptionType) -> Option<&Exception> {
        if matches!(typ, ExceptionType::Reserved(_)) {
            return None;
        }
        let excp_num: u32 = typ.into();
        let excp_num = excp_num as usize;
        if excp_num < 16 {
            Some(&self.internal[excp_num])
        } else {
            self.external.get(excp_num - 16)
        }
    }

    /// add an exception to the pending queue,
    /// reordering the queue as necessary based on priority
    pub fn queue_exception(&mut self, typ: ExceptionType) {
        todo!()
    }

    /// pop the next exception to service from the pending queue
    pub fn pop_exception(&mut self) -> Option<ExceptionType> {
        todo!()
    }

    /// check for exception of higher priority than currently being serviced
    pub fn preempt_pending(&self) -> bool {
        todo!()
    }

    /// check for any pending exception
    pub fn pending(&self) -> bool {
        !self.queue.is_empty()
    }

    /// current exception priority
    /// from B1.5.4 page B1-529
    pub fn current_priority(&self,
        scs: &SysCtrlSpace,
    ) -> i16 {
        // priority x
        let highestpri = 256;
        let boostedpri = 256;
        let subgroupshift = todo!();
        todo!()
    }
}

/// Interrupt Set-Enable Registers.
/// Enables or reads the enable state of a group of interrupts.
/// Word-accessible only. 
/// 
/// software can enable multiple interrupts in a single write to ISER
///
/// See B3.4.6
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ISER {
    /// for register NVIC_ISERn, enables or shows current enabled state 
    /// of interrupt (m + (32 * n)) for each bit m in setena.
    /// write (0 = no effect, 1 = enable interrupt)
    /// read (0 = interrupt disabled, 1 = interrupt enabled)
    /// 
    /// note: n = 15, only enables lower 16 bits, upper 16 bits are RAZ/WI
    #[bits(32)]
    pub setena: u32, 
}

/// Interrupt Clear-Enable Registers.
/// Disables or reads the enable state of a group of interrupts.
/// Word-accessible only.
/// 
/// software can disable multiple interrupts in a single write to ICER
///
/// See B3.4.7
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ICER {
    /// for register NVIC_ICERn, disable or shows the current enabled state
    /// of interrupt (m + (32 * n)) for each bit m in clrena.
    /// write (0 = no effect, 1 = disable interupt)
    /// read (0 = interrupt disabled, 1 = interrupt enabled)
    /// 
    /// note: for n = 15, only enables lower 16 bits, upper 16 bits are RAZ/WI
    #[bits(32)]
    pub clrena: u32,
}

/// Interrupt Set-Pending Registers.
/// Changes the state of a group of interrupts to pending or shows the pending state.
/// Word-accessible only.
/// 
/// software can set multiple interrupts to pending state in a signle write to ISPR
///
/// See B3.4.5
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ISPR {
    /// for register NVIC_ISPRn, changes state of interrupt (m + (32 * n)) to
    /// pending, or shows whether state of interrupt is pending
    /// write (0 = no effect, 1 = set interrupt pending)
    /// read (0 = interrupt not pending, 1 = interrupt pending)
    /// 
    /// note: n = 15, only enables lower 16 bits, upper 16 bits are RAZ/WI
    #[bits(32)]
    pub setpend: u32,
}

/// Interrupt Clear-Pending Registers
/// Clears the pending state of a group of interrupts or shows the pending state.
/// Word-accessible only.
///
/// software can clear pending state of multiple interrupts in single write to ICPR
/// 
/// See B3.4.4
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ICPR {
    /// for register NVIC_ICPRn, clears pending state of interrupt (m + (32 * n))
    /// or shows whether interrupt is pending.
    /// write (0 = no effect, 1 = clear interrupt pending)
    /// read (0 = interrupt not pending, 1 = interrupt pending)
    /// 
    /// note: n = 15, only enables lower 16 bits, upper 16 bits are RAZ/WI
    #[bits(32)]
    pub clrpend: u32,
}

/// Interrupt Active Bit Registers.
/// Shows whether each interrupt in a group of 32 is active.
/// Word-accessible only. Read-only.
/// 
/// See B3.4.8
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct IABR {
    /// for register NVIC_IABRn, shows whether interrupt (m + (32 * n)) is active
    /// (0 = interrupt not active, 1 = interrupt active)
    /// 
    /// note: n = 15, upper 16 bits are RAZ/WI
    #[bits(32)]
    pub active: u32,
}

/// Interrupt Priority Registers.
/// Sets or reads interrupt priorities.
/// Byte, halfword, and word accessible.
///
/// See B3.4.9
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct IPR {
    /// Priority of interrupt number 4n + 3.
    #[bits(8)]
    pub pri_n3: u32,
    /// Priority of interrupt number 4n + 2.
    #[bits(8)]
    pub pri_n2: u32,
    /// Priority of interrupt number 4n + 1.
    #[bits(8)]
    pub pri_n1: u32,
    /// Priority of interrupt number 4n.
    #[bits(8)]
    pub pri_n0: u32,
}









impl<'a> NVICRegs<'a> {
    fn _view_bytes(&self, word_offset: usize) -> &[u8; 4] {
        assert!(word_offset < self.backing.len());
        unsafe {
            &*(&self.backing[word_offset] as *const u32 as *const [u8; 4])
        }
    }

    fn _view_bytes_mut(&mut self, word_offset: usize) -> &mut [u8; 4] {
        assert!(word_offset < self.backing.len());
        unsafe {
            &mut *(&mut self.backing[word_offset] as *mut u32 as *mut [u8; 4])
        }
    }

    
    // scs registers

    pub fn icsr(&self) -> &ICSR {
        let word_offset = SCRegType::ICSR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ICSR) }
    }

    pub fn vtor(&self) -> &VTOR {
        let word_offset = SCRegType::VTOR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const VTOR) }
    }

    pub fn aircr(&self) -> &AIRCR {
        let word_offset = SCRegType::AIRCR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const AIRCR) }
    }

    pub fn scr(&self) -> &SCR {
        let word_offset = SCRegType::SCR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SCR) }
    }

    pub fn shpr1(&self) -> &SHPR1 {
        let word_offset = SCRegType::SHPR1(0).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHPR1) }
    }

    pub fn shpr2(&self) -> &SHPR2 {
        let word_offset = SCRegType::SHPR2(0).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHPR2) }
    }

    pub fn shpr3(&self) -> &SHPR3 {
        let word_offset = SCRegType::SHPR3(0).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHPR3) }
    }

    pub fn shcsr(&self) -> &SHCSR {
        let word_offset = SCRegType::SHCSR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHCSR) }
    }

    pub fn cfsr(&self) -> &CFSR {
        let word_offset = SCRegType::CFSR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CFSR) }
    }

    pub fn hfsr(&self) -> &HFSR {
        let word_offset = SCRegType::HFSR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const HFSR) }
    }

    pub fn ictr(&self) -> &ICTR {
        let word_offset = SCRegType::ICTR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ICTR) }
    }

    pub fn stir(&self) -> &STIR {
        let word_offset = SCRegType::STIR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const STIR) }
    }

    pub fn icsr_mut(&mut self) -> &mut ICSR {
        let word_offset = SCRegType::ICSR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ICSR) }
    }
    
    pub fn vtor_mut(&mut self) -> &mut VTOR {
        let word_offset = SCRegType::VTOR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut VTOR) }
    }
    
    pub fn aircr_mut(&mut self) -> &mut AIRCR {
        let word_offset = SCRegType::AIRCR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut AIRCR) }
    }
    
    pub fn scr_mut(&mut self) -> &mut SCR {
        let word_offset = SCRegType::SCR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SCR) }
    }
    
    pub fn shpr1_mut(&mut self) -> &mut SHPR1 {
        let word_offset = SCRegType::SHPR1(0).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHPR1) }
    }
    
    pub fn shpr2_mut(&mut self) -> &mut SHPR2 {
        let word_offset = SCRegType::SHPR2(0).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHPR2) }
    }
    
    pub fn shpr3_mut(&mut self) -> &mut SHPR3 {
        let word_offset = SCRegType::SHPR3(0).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHPR3) }
    }
    
    pub fn shcsr_mut(&mut self) -> &mut SHCSR {
        let word_offset = SCRegType::SHCSR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHCSR) }
    }
    
    pub fn cfsr_mut(&mut self) -> &mut CFSR {
        let word_offset = SCRegType::CFSR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CFSR) }
    }
    
    pub fn hfsr_mut(&mut self) -> &mut HFSR {
        let word_offset = SCRegType::HFSR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut HFSR) }
    }
    
    pub fn stir_mut(&mut self) -> &mut STIR {
        let word_offset = SCRegType::STIR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut STIR) }
    }

    // nvic registers

    pub fn get_iser(&self, n: u8) -> &ISER {
        let word_offset = NVICRegType::ISER(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ISER) }
    }

    pub fn get_icer(&self, n: u8) -> &ICER {
        let word_offset = NVICRegType::ICER(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ICER) }
    }

    pub fn get_ispr(&self, n: u8) -> &ISPR {
        let word_offset = NVICRegType::ISPR(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ISPR) }
    }

    pub fn get_icpr(&self, n: u8) -> &ICPR {
        let word_offset = NVICRegType::ICPR(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ICPR) }
    }

    pub fn get_iabr(&self, n: u8) -> &IABR {
        let word_offset = NVICRegType::IABR(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const IABR) }
    }

    pub fn get_ipr(&self, n: u8) -> &IPR {
        let word_offset = NVICRegType::IPR(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const IPR) }
    }

    pub fn get_iser_mut(&mut self, n: u8) -> &mut ISER {
        let word_offset = NVICRegType::ISER(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ISER) }
    }

    pub fn get_icer_mut(&mut self, n: u8) -> &mut ICER {
        let word_offset = NVICRegType::ICER(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ICER) }
    }

    pub fn get_ispr_mut(&mut self, n: u8) -> &mut ISPR {
        let word_offset = NVICRegType::ISPR(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ISPR) }
    }
    
    pub fn get_icpr_mut(&mut self, n: u8) -> &mut ICPR {
        let word_offset = NVICRegType::ICPR(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ICPR) }
    }
    
    pub fn get_iabr_mut(&mut self, n: u8) -> &mut IABR {
        let word_offset = NVICRegType::IABR(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut IABR) }
    }
    
    pub fn get_ipr_mut(&mut self, n: u8) -> &mut IPR {
        let word_offset = NVICRegType::IPR(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut IPR) }
    }
}