//! nvic.rs
//! 
//! implementation of the nested vector interrupt controller for armv7m
use bitfield_struct::bitfield;

use crate::backend;
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

    pub fn reset_value(&self) -> Option<u32> {
        // all have reset value 0
        Some(0)
    }
}

#[derive(Debug)]
pub struct NVICRegs<'a> {
    backing: &'a [u32; 0x340],
}

impl<'a> NVICRegs<'a> {
    pub fn new(backing: &'a [u32; 0x340]) -> Self {
        Self { backing }
    }
}

#[derive(Debug)]
pub struct NVICRegsMut<'a> {
    backing: &'a mut [u32; 0x340],
}

impl<'a> NVICRegsMut<'a> {
    pub fn new(backing: &'a mut [u32; 0x340]) -> Self {
        Self { backing }
    }

    /// perform an event-triggering read of nvic register bytes
    pub fn read_bytes(
        &mut self,
        offset: usize,
        dst: &mut [u8],
        _events: &mut VecDeque<Event>,
    ) -> Result<(), backend::Error> {
        let word_offset = offset / 4;
        let address = BASE + offset as u32;
        let reg_type = NVICRegType::lookup_offset(offset)
            .ok_or_else(| | {
                backend::Error::from(Error::InvalidSysCtrlReg(address.into()))
            })?;
        match reg_type {
            NVICRegType::IPR(_n) => {
                check_alignment(address, dst.len(), Alignment::Any)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
            }
            _ => {
                check_alignment(address, dst.len(), Alignment::Word)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
            }
        }
        let reg_slice = self.view_bytes(word_offset);
        let byte_offset = offset & 0b11;
        dst.copy_from_slice(&reg_slice[byte_offset..]);
        Ok(())
    }

    /// perform an event-triggering write of nvic register bytes
    pub fn write_bytes(
        &mut self,
        offset: usize,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), backend::Error> {
        check_alignment(BASE + offset as u32, src.len(), Alignment::Any)
            .map_err(|(address, size, expected)| {
                Error::AlignmentViolation(address, size, expected)
            })?;
        let word_offset = offset / 4;
        let reg_type = NVICRegType::lookup_offset(offset)
            .ok_or_else( | | {
                let address = Address::from(BASE + offset as u32);
                backend::Error::from(Error::InvalidSysCtrlReg(address))
            })?;
        let write_val = src.iter().enumerate().take(4)
            .fold(0u32, |val, (i, &byte)| {
                val | ((byte as u32) << i)
            });
        match reg_type {
            NVICRegType::IPR(n) => {
                let byte_offset = offset & 0b11;
                let slice = self.view_bytes_mut(word_offset);
                let slice = &mut slice[byte_offset..];
                for (i, val) in src.iter().enumerate() {
                    slice[i] = *val;
                    let excp = ExceptionType::from((4 * n + byte_offset as u8) as u32);
                    events.push_back(Event::ExceptionSetPriority(excp, *val))
                }
            }
            NVICRegType::ISER(n) => {
                check_alignment(BASE + offset as u32, src.len(), Alignment::Word)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
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
                check_alignment(BASE + offset as u32, src.len(), Alignment::Word)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
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
                check_alignment(BASE + offset as u32, src.len(), Alignment::Word)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
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
                check_alignment(BASE + offset as u32, src.len(), Alignment::Word)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
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
                return Err(backend::Error::from(err).into());
            }
        }
        Ok(())
    }
}

pub trait NVIC {
    fn view_bytes(&self, word_offset: usize) -> &[u8; 4];

    // scs registers

    fn icsr(&self) -> &ICSR {
        let word_offset = SCRegType::ICSR.offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const ICSR) }
    }

    fn vtor(&self) -> &VTOR {
        let word_offset = SCRegType::VTOR.offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const VTOR) }
    }

    fn aircr(&self) -> &AIRCR {
        let word_offset = SCRegType::AIRCR.offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const AIRCR) }
    }

    fn scr(&self) -> &SCR {
        let word_offset = SCRegType::SCR.offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const SCR) }
    }

    fn shpr1(&self) -> &SHPR1 {
        let word_offset = SCRegType::SHPR1(0).offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const SHPR1) }
    }

    fn shpr2(&self) -> &SHPR2 {
        let word_offset = SCRegType::SHPR2(0).offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const SHPR2) }
    }

    fn shpr3(&self) -> &SHPR3 {
        let word_offset = SCRegType::SHPR3(0).offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const SHPR3) }
    }

    fn shcsr(&self) -> &SHCSR {
        let word_offset = SCRegType::SHCSR.offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const SHCSR) }
    }

    fn cfsr(&self) -> &CFSR {
        let word_offset = SCRegType::CFSR.offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const CFSR) }
    }

    fn hfsr(&self) -> &HFSR {
        let word_offset = SCRegType::HFSR.offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const HFSR) }
    }

    fn ictr(&self) -> &ICTR {
        let word_offset = SCRegType::ICTR.offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const ICTR) }
    }

    fn stir(&self) -> &STIR {
        let word_offset = SCRegType::STIR.offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const STIR) }
    }

    // nvic registers

    fn get_iser(&self, n: u8) -> &ISER {
        let word_offset = NVICRegType::ISER(n).offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const ISER) }
    }

    fn get_icer(&self, n: u8) -> &ICER {
        let word_offset = NVICRegType::ICER(n).offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const ICER) }
    }

    fn get_ispr(&self, n: u8) -> &ISPR {
        let word_offset = NVICRegType::ISPR(n).offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const ISPR) }
    }

    fn get_icpr(&self, n: u8) -> &ICPR {
        let word_offset = NVICRegType::ICPR(n).offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const ICPR) }
    }

    fn get_iabr(&self, n: u8) -> &IABR {
        let word_offset = NVICRegType::IABR(n).offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const IABR) }
    }

    fn get_ipr(&self, n: u8) -> &IPR {
        let word_offset = NVICRegType::IPR(n).offset() / 4;
        unsafe { &*(self.view_bytes(word_offset) as *const [u8; 4] as *const u32 as *const IPR) }
    }
}

pub trait NVICMut: NVIC {
    fn view_bytes_mut(&mut self, _word_offset: usize) -> &mut [u8; 4];

    fn icsr_mut(&mut self) -> &mut ICSR {
        let word_offset = SCRegType::ICSR.offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut ICSR) }
    }
    
    fn vtor_mut(&mut self) -> &mut VTOR {
        let word_offset = SCRegType::VTOR.offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut VTOR) }
    }
    
    fn aircr_mut(&mut self) -> &mut AIRCR {
        let word_offset = SCRegType::AIRCR.offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut AIRCR) }
    }
    
    fn scr_mut(&mut self) -> &mut SCR {
        let word_offset = SCRegType::SCR.offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut SCR) }
    }
    
    fn shpr1_mut(&mut self) -> &mut SHPR1 {
        let word_offset = SCRegType::SHPR1(0).offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut SHPR1) }
    }
    
    fn shpr2_mut(&mut self) -> &mut SHPR2 {
        let word_offset = SCRegType::SHPR2(0).offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut SHPR2) }
    }
    
    fn shpr3_mut(&mut self) -> &mut SHPR3 {
        let word_offset = SCRegType::SHPR3(0).offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut SHPR3) }
    }
    
    fn shcsr_mut(&mut self) -> &mut SHCSR {
        let word_offset = SCRegType::SHCSR.offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut SHCSR) }
    }
    
    fn cfsr_mut(&mut self) -> &mut CFSR {
        let word_offset = SCRegType::CFSR.offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut CFSR) }
    }
    
    fn hfsr_mut(&mut self) -> &mut HFSR {
        let word_offset = SCRegType::HFSR.offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut HFSR) }
    }
    
    fn stir_mut(&mut self) -> &mut STIR {
        let word_offset = SCRegType::STIR.offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut STIR) }
    }

    fn get_iser_mut(&mut self, n: u8) -> &mut ISER {
        let word_offset = NVICRegType::ISER(n).offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut ISER) }
    }

    fn get_icer_mut(&mut self, n: u8) -> &mut ICER {
        let word_offset = NVICRegType::ICER(n).offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut ICER) }
    }

    fn get_ispr_mut(&mut self, n: u8) -> &mut ISPR {
        let word_offset = NVICRegType::ISPR(n).offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut ISPR) }
    }
    
    fn get_icpr_mut(&mut self, n: u8) -> &mut ICPR {
        let word_offset = NVICRegType::ICPR(n).offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut ICPR) }
    }
    
    fn get_iabr_mut(&mut self, n: u8) -> &mut IABR {
        let word_offset = NVICRegType::IABR(n).offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut IABR) }
    }
    
    fn get_ipr_mut(&mut self, n: u8) -> &mut IPR {
        let word_offset = NVICRegType::IPR(n).offset() / 4;
        unsafe { &mut *(self.view_bytes_mut(word_offset) as *mut [u8; 4] as *mut u32 as *mut IPR) }
    }
}

impl<'a> NVIC for NVICRegs<'a> {
    fn view_bytes(&self, word_offset: usize) -> &[u8; 4] {
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const [u8; 4]) }
    }
}

impl<'a> NVIC for NVICRegsMut<'a> {
    fn view_bytes(&self, word_offset: usize) -> &[u8; 4] {
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const [u8; 4]) }
    }
}

impl<'a> NVICMut for NVICRegsMut<'a> {
    fn view_bytes_mut(&mut self, word_offset: usize) -> &mut [u8; 4] {
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut [u8; 4]) }
    }
}

#[allow(unused)]
/// state for nested vector interrupt controller
#[derive(Default, Debug, Clone)]
pub struct NVICState {
    enabled: Vec<ExceptionType>,
    pending: Vec<ExceptionType>,
    active: Vec<ExceptionType>,
}

impl NVICState {

    /// enable an exception
    #[instrument(skip_all)]
    pub fn enable(&mut self, typ: ExceptionType) {
        debug!("enable {typ:?}");
        let mut idx = 0;
        for t in self.enabled.iter() {
            if *t < typ {
                idx += 1;
                continue;
            } else if *t == typ {
                // exception already enabled
                return;
            } else {
                break;
            }
        }
        self.enabled.insert(idx, typ);
    }

    /// disable an exception
    #[instrument(skip_all)]
    pub fn disable(&mut self, typ: ExceptionType) {
        debug!("disable {typ:?}");
        let idx = self.enabled.iter()
            .position(|t| *t == typ);
        if let Some(idx) = idx {
            self.enabled.remove(idx);
        }
    }

    /// set an exception as pending
    #[instrument(skip_all)]
    pub fn set_pending(
        &mut self,
        typ: ExceptionType,
    ) {
        debug!("set pending {typ:?}");
        let mut idx = 0;
        for t in self.pending.iter() {
            if *t < typ {
                idx += 1;
                continue;
            } else if *t == typ {
                // exception already pending
                return;
            } else {
                break;
            }
        }
        self.pending.insert(idx, typ);
    }

    /// clr a pending interrupt (does nothing if not pending)
    /// will not reorder exception queue
    #[instrument(skip_all)]
    pub fn clr_pending(
        &mut self,
        typ: ExceptionType,
    ) {
        debug!("clr pending {typ:?}");
        let idx = self.pending.iter()
            .position(|t| *t == typ);
        if let Some(idx) = idx {
            self.pending.remove(idx);
        }
    }

    /// set an exception as active
    /// exception will not be set active unless it is enabled per B3.4.1
    #[instrument(skip_all)]
    pub fn set_active(
        &mut self,
        typ: ExceptionType,
    ) {
        debug!("set active {typ:?}");
        assert!(self.pending.contains(&typ),
            "interrupt must be pending before becoming active");
        self.clr_pending(typ);
        let mut idx = 0;
        for t in self.active.iter() {
            if *t < typ {
                idx += 1;
                continue;
            } else if *t == typ {
                return;
            } else {
                break;
            }
        }
        self.active.insert(idx, typ);
    }

    /// clear an exception
    #[instrument(skip_all)]
    pub fn clr_active(
        &mut self,
        typ: ExceptionType,
    ) {
        debug!("clr active {typ:?}");
        let idx = self.active.iter()
            .position(|t| *t == typ);
        if let Some(idx) = idx {
            self.active.remove(idx);
        }
    }

    pub fn active(&self) -> &[ExceptionType] {
        &self.active
    }

    pub fn pending(&self) -> &[ExceptionType] {
        &self.pending
    }

    pub fn enabled(&self) -> &[ExceptionType] {
        &self.enabled
    }
}

impl SysCtrlSpace {
    pub fn set_exception_pending(&mut self, typ: ExceptionType) {
        self.nvic.set_pending(typ);
        if let Some(typ) = self.nvic.pending().first().cloned() {
            self.get_icsr_mut().set_vectpending(u32::from(&typ));
        }
    }

    pub fn enable_exception(&mut self, typ: ExceptionType) {
        self.nvic.enable(typ)
    }

    pub fn disable_exception(&mut self, typ: ExceptionType) {
        self.nvic.disable(typ)
    }

    pub fn clr_exception_pending(&mut self, typ: ExceptionType) {
        self.nvic.clr_pending(typ)
    }

    pub fn set_exception_active(&mut self, typ: ExceptionType) {
        self.nvic.set_active(typ)
    }

    pub fn clr_exception_active(&mut self, typ: ExceptionType) {
        self.nvic.clr_active(typ)
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
    pub pri_n3: u8,
    /// Priority of interrupt number 4n + 2.
    #[bits(8)]
    pub pri_n2: u8,
    /// Priority of interrupt number 4n + 1.
    #[bits(8)]
    pub pri_n1: u8,
    /// Priority of interrupt number 4n.
    #[bits(8)]
    pub pri_n0: u8,
}

impl IPR {
    pub fn pri_n(&self, offset: u8) -> u8 {
        match offset {
            0 => { self.pri_n0() }
            1 => { self.pri_n1() }
            2 => { self.pri_n2() }
            3 => { self.pri_n3() }
            _ => { unreachable!("pri_n offset must be < 4") }
        }
    }

    pub fn set_pri_n(&mut self, offset: u8, value: u8) {
        match offset {
            0 => { self.0 = self.with_pri_n0(value).0 }
            1 => { self.0 = self.with_pri_n1(value).0 }
            2 => { self.0 = self.with_pri_n2(value).0 }
            3 => { self.0 = self.with_pri_n3(value).0 }
            _ => { unreachable!("pri_n offset must be < 4") }
        }
    }
}

