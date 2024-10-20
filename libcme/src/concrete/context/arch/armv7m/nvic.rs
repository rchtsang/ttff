//! nvic.rs
//! 
//! implementation of the nested vector interrupt controller for armv7m

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NVICReg {

}

impl NVICReg {
    pub fn read_evt(&self, read_val: u32) -> Result<Vec<Event>, Error> {
        todo!()
    }

    pub fn write_evt(&self, write_val: u32) -> Result<Vec<Event>, Error> {
        todo!()
    }

    pub fn lookup_offset(offset: usize) -> Option<NVICReg> {
        todo!() // see B3.4.3 for implementation
    }
}


/// state for nested vector interrupt controller
#[derive(Debug, Clone)]
pub struct NVIC {
    pub(crate) vtsize: usize,

    pub(crate) internal: [Exception; 16],
    pub(crate) external: Vec<Exception>,
    pub(crate) queue: Vec<ExceptionType>,
    pub(crate) active: Vec<ExceptionType>,
}

impl NVIC {
    /// construct a new nvic instance from a given vector table slice
    pub fn new_with(vt: &[u8]) -> Self {
        assert!(vt.len() >= 16, "vector table must have arch-defined exceptions");
        let vtsize = vt.len();
        let internal = [
            Exception::default(),
            Exception::new_with(ExceptionType::Reset,         -3, &vt[( 1 * 4)..(( 1 + 1) * 4)]),
            Exception::new_with(ExceptionType::NMI,           -2, &vt[( 2 * 4)..(( 2 + 1) * 4)]),
            Exception::new_with(ExceptionType::HardFault,     -1, &vt[( 3 * 4)..(( 3 + 1) * 4)]),
            Exception::new_with(ExceptionType::MemFault,       0, &vt[( 4 * 4)..(( 4 + 1) * 4)]),
            Exception::new_with(ExceptionType::BusFault,       0, &vt[( 5 * 4)..(( 5 + 1) * 4)]),
            Exception::new_with(ExceptionType::UsageFault,     0, &vt[( 6 * 4)..(( 6 + 1) * 4)]),
            Exception::new_with(ExceptionType::Reserved(7),    0, &vt[( 7 * 4)..(( 7 + 1) * 4)]),
            Exception::new_with(ExceptionType::Reserved(8),    0, &vt[( 8 * 4)..(( 8 + 1) * 4)]),
            Exception::new_with(ExceptionType::Reserved(9),    0, &vt[( 9 * 4)..(( 9 + 1) * 4)]),
            Exception::new_with(ExceptionType::Reserved(10),   0, &vt[(10 * 4)..((10 + 1) * 4)]),
            Exception::new_with(ExceptionType::SVCall,         0, &vt[(11 * 4)..((11 + 1) * 4)]),
            Exception::new_with(ExceptionType::DebugMonitor,   0, &vt[(12 * 4)..((12 + 1) * 4)]),
            Exception::new_with(ExceptionType::Reserved(13),   0, &vt[(13 * 4)..((13 + 1) * 4)]),
            Exception::new_with(ExceptionType::PendSV,         0, &vt[(14 * 4)..((14 + 1) * 4)]),
            Exception::new_with(ExceptionType::SysTick,        0, &vt[(15 * 4)..((15 + 1) * 4)]),
        ];
        let mut external = vec![];
        for (i, entry) in vt.chunks(4).enumerate() {
            let typ = ExceptionType::ExternalInterrupt(i as u32 + 16);
            let priority = 0;
            let e = Exception::new_with(typ, priority, entry);
            external.push(e);
        }
        let queue = vec![];
        let active = vec![];
        Self { vtsize, internal, external, queue, active }
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

