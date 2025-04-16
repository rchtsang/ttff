//! coproc.rs
//! 
//! coproc userops

use crate::backend;
use super::*;

pub(super) fn _coprocessor_function(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_function2(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_load(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_load2(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_loadlong(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_loadlong2(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_moveto(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_moveto2(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_movefrom_rt(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_movefrom_rt2(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_movefrom2(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

/// STC and STC2 instruction
/// 
/// see ARMTHUMBinstructions.sinc and A7.7.158
/// 
/// stores data from a coprocessor to a sequence of consecutive 
/// memory addresses. if no coprocessor can execute the instruction, 
/// a UsageFault exception is generated.
pub(super) fn _coprocessor_store(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

/// STC2 instruction
/// 
/// see ARMinstructions.sinc, only defined for VERSION_5
pub(super) fn _coprocessor_store2(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_storelong(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coprocessor_storelong2(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_main_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_cache_type(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_tcm_status(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_tlb_type(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_auxiliary_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_coprocessor_access_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_secure_configuration(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_secure_debug_enable(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_non_secure_access_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_translation_table_base_0(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_translation_table_base_1(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_translation_table_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_domain_access_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_data_fault_status(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_instruction_fault_status(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_instruction_fault_address(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_fault_address(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_instruction_fault(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_wait_for_interrupt(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_invalidate_entire_instruction(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_invalidate_instruction_cache_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_flush_prefetch_buffer(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_invalidate_entire_data_cache(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_invalidate_entire_data_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_invalidate_entire_data_by_index(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_clean_entire_data_cache(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_clean_data_cache_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_clean_data_cache_by_index(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_data_synchronization(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_data_memory_barrier(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_invalidate_data_cache_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_invalidate_unified_tlb_unlocked(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_invalidate_unified_tlb_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_invalidate_unified_tlb_by_asid_match(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_fcse_pid(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_backend_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_user_rw_thread_and_process_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_user_r_thread_and_process_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_privileged_only_thread_and_process_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_peripherial_port_memory_remap(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_feature_identification(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_isa_feature_identification(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_peripheral_port_memory_remap(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_control_registers(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_security_world_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_translation_table(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_instruction_cache(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_data_cache_operations(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_identification_registers(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_moveto_peripheral_system(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_main_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_cache_type(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_tcm_status(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_tlb_type(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_auxiliary_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_coprocessor_access_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_secure_configuration(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_secure_debug_enable(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_non_secure_access_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_translation_table_base_0(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_translation_table_base_1(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_translation_table_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_domain_access_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_data_fault_status(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_instruction_fault(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_fault_address(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_instruction_fault_status(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_instruction_fault_address(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_wait_for_interrupt(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_invalidate_entire_instruction(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_invalidate_instruction_cache_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_flush_prefetch_buffer(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_invalidate_entire_data_cache(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_invalidate_entire_data_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_invalidate_entire_data_by_index(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_clean_entire_data_cache(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_clean_data_cache_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_clean_data_cache_by_index(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_data_synchronization(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_data_memory_barrier(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_invalidate_data_cache_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_invalidate_unified_tlb_unlocked(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_invalidate_unified_tlb_by_mva(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_invalidate_unified_tlb_by_asid_match(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_fcse_pid(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_backend_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_user_rw_thread_and_process_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_user_r_thread_and_process_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_privileged_only_thread_and_process_id(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_peripherial_port_memory_remap(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_feature_identification(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_isa_feature_identification(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_peripheral_port_memory_remap(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_control_registers(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_security_world_control(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_translation_table(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_instruction_cache(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_data_cache_operations(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_identification_registers(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

pub(super) fn _coproc_movefrom_peripheral_system(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

