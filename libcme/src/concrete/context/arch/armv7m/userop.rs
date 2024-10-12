//! userop.rs
//! 
//! implementations of armv7m pcode userops.
//! strongly coupled to parent module.
//! 
//! implement userops as a function table
#![allow(unused)]
#![allow(dead_code)]

use super::*;

impl<'irb> Context<'irb> {

    pub fn _userop(&mut self,
        index: usize,
        inputs: &[VarnodeData],
        output: Option<&VarnodeData>,
    ) -> Result<Option<Location>, context::Error> {
        if index >= USEROPS.len() {
            return Err(Error::InvalidUserOp(index).into());
        }
        USEROPS[index].1(self, index, inputs, output).map_err(|err| err.into())
    }
}

/// ghidra userop function table
static USEROPS: &'static [(
    &'static str,
    fn(&mut Context, usize, &[VarnodeData], Option<&VarnodeData>) -> Result<Option<Location>, Error>,
)] = &[
    ("count_leading_zeroes",              _count_leading_zeroes),
    ("coprocessor_function",              _coprocessor_function),
    ("coprocessor_function2",             _coprocessor_function2),
    ("coprocessor_load",                  _coprocessor_load),
    ("coprocessor_load2",                 _coprocessor_load2),
    ("coprocessor_loadlong",              _coprocessor_loadlong),
    ("coprocessor_loadlong2",             _coprocessor_loadlong2),
    ("coprocessor_moveto",                _coprocessor_moveto),
    ("coprocessor_moveto2",               _coprocessor_moveto2),
    ("coprocessor_movefromRt",            _coprocessor_movefrom_rt),
    ("coprocessor_movefromRt2",           _coprocessor_movefrom_rt2),
    ("coprocessor_movefrom2",             _coprocessor_movefrom2),
    ("coprocessor_store",                 _coprocessor_store),
    ("coprocessor_store2",                _coprocessor_store2),
    ("coprocessor_storelong",             _coprocessor_storelong),
    ("coprocessor_storelong2",            _coprocessor_storelong2),
    ("software_interrupt",                _software_interrupt),
    ("software_bkpt",                     _software_bkpt),
    ("software_udf",                      _software_udf),
    ("software_hlt",                      _software_hlt),
    ("software_hvc",                      _software_hvc),
    ("software_smc",                      _software_smc),
    ("setUserMode",                       _set_user_mode),
    ("setFIQMode",                        _set_fiq_mode),
    ("setIRQMode",                        _set_irq_mode),
    ("setSupervisorMode",                 _set_supervisor_mode),
    ("setMonitorMode",                    _set_monitor_mode),
    ("setAbortMode",                      _set_abort_mode),
    ("setUndefinedMode",                  _set_undefined_mode),
    ("setSystemMode",                     _set_system_mode),
    ("enableIRQinterrupts",               _enable_irq_interrupts),
    ("enableFIQinterrupts",               _enable_fiq_interrupts),
    ("enableDataAbortInterrupts",         _enable_dataabort_interrupts),
    ("disableIRQinterrupts",              _disable_irq_interrupts),
    ("disableFIQinterrupts",              _disable_fiq_interrupts),
    ("isFIQinterruptsEnabled",            _is_fiq_interrupts_enabled),
    ("isIRQinterruptsEnabled",            _is_irq_interripts_enabled),
    ("disableDataAbortInterrupts",        _disable_dataabort_interrupts),
    ("hasExclusiveAccess",                _has_exclusive_access),
    ("isCurrentModePrivileged",           _is_current_mode_privileged),
    ("setThreadModePrivileged",           _set_thread_mode_privileged),
    ("isThreadMode",                      _is_thread_mode),
    ("jazelle_branch",                    _jazelle_branch),
    ("ClearExclusiveLocal",               _clear_exclusive_local),
    ("HintDebug",                         _hint_debug),
    ("DataMemoryBarrier",                 _data_memory_barrier),
    ("DataSynchronizationBarrier",        _data_synchronization_barrier),
    ("secureMonitorCall",                 _secure_monitor_call),
    ("WaitForEvent",                      _wait_for_event),
    ("WaitForInterrupt",                  _wait_for_interrupt),
    ("HintYield",                         _hint_yield),
    ("InstructionSynchronizationBarrier", _instruction_synchronization_barrier),
    ("HintPreloadData",                   _hint_preload_data),
    ("HintPreloadDataForWrite",           _hint_preload_data_for_write),
    ("HintPreloadInstruction",            _hint_preload_instruction),
    ("SignedSaturate",                    _signed_saturate),
    ("SignedDoesSaturate",                _signed_does_saturate),
    ("UnsignedSaturate",                  _unsigned_saturate),
    ("UnsignedDoesSaturate",              _unsigned_does_saturate),
    ("Absolute",                          _absolute),
    ("ReverseBitOrder",                   _reverse_bit_order),
    ("SendEvent",                         _send_event),
    ("setEndianState",                    _set_endian_state),
];

fn _count_leading_zeroes(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_function(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_function2(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_load(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_load2(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_loadlong(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_loadlong2(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_moveto(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_moveto2(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_movefrom_rt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_movefrom_rt2(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_movefrom2(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_store(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_store2(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_storelong(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _coprocessor_storelong2(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _software_interrupt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _software_bkpt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _software_udf(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _software_hlt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _software_hvc(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _software_smc(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_user_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_fiq_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_irq_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_supervisor_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_monitor_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_abort_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_undefined_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_system_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _enable_irq_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _enable_fiq_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _enable_dataabort_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _disable_irq_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _disable_fiq_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _is_fiq_interrupts_enabled(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _is_irq_interripts_enabled(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _disable_dataabort_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _has_exclusive_access(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _is_current_mode_privileged(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_thread_mode_privileged(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _is_thread_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _jazelle_branch(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _clear_exclusive_local(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _hint_debug(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _data_memory_barrier(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _data_synchronization_barrier(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _secure_monitor_call(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _wait_for_event(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _wait_for_interrupt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _hint_yield(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _instruction_synchronization_barrier(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _hint_preload_data(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _hint_preload_data_for_write(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _hint_preload_instruction(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _signed_saturate(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _signed_does_saturate(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _unsigned_saturate(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _unsigned_does_saturate(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _absolute(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _reverse_bit_order(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _send_event(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}

fn _set_endian_state(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, Error> {
    todo!("unsupported userop: {}", USEROPS[index].0)
}
