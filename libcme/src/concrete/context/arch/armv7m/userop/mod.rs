//! userop module
//! 
//! implementations of armv7m pcode userops.
//! strongly coupled to parent module.
//! 
//! implement userops as a function table
#![allow(unused)]

use crate::concrete::eval::bool2bv;

use super::*;

use crate::utils::*;

impl<'irb> Context<'irb> {

    pub fn _userop(&mut self,
        index: usize,
        inputs: &[VarnodeData],
        output: Option<&VarnodeData>,
    ) -> Result<Option<Location>, context::Error> {
        let userop = _lookup_userop(index);
        userop.call(self, inputs, output)
    }
}

mod coproc;
use coproc::*;
mod vector;
use vector::*;

// notes:
// - if a userop call triggers events, it should add the events to the 
//   context's event queue (context.events)

pub struct UserOp {
    pub index: usize,
    pub name: &'static str,
    pub func: fn(&mut Context, usize, &[VarnodeData], Option<&VarnodeData>) -> Result<Option<Location>, context::Error>,
}

impl UserOp {
    pub fn call(&self,
        context: &mut Context,
        inputs: &[VarnodeData],
        output: Option<&VarnodeData>,
    ) -> Result<Option<Location>, context::Error> {
        (self.func)(context, self.index, inputs, output)
    }
}

/// ghidra userop lookup table
fn _lookup_userop(index: usize) -> &'static UserOp {
    match index {
        0 => { &UserOp { index:   0, name: "count_leading_zeroes",                            func: _count_leading_zeroes } }
        1 => { &UserOp { index:   1, name: "coprocessor_function",                            func: _coprocessor_function } }
        2 => { &UserOp { index:   2, name: "coprocessor_function2",                           func: _coprocessor_function2 } }
        3 => { &UserOp { index:   3, name: "coprocessor_load",                                func: _coprocessor_load } }
        4 => { &UserOp { index:   4, name: "coprocessor_load2",                               func: _coprocessor_load2 } }
        5 => { &UserOp { index:   5, name: "coprocessor_loadlong",                            func: _coprocessor_loadlong } }
        6 => { &UserOp { index:   6, name: "coprocessor_loadlong2",                           func: _coprocessor_loadlong2 } }
        7 => { &UserOp { index:   7, name: "coprocessor_moveto",                              func: _coprocessor_moveto } }
        8 => { &UserOp { index:   8, name: "coprocessor_moveto2",                             func: _coprocessor_moveto2 } }
        9 => { &UserOp { index:   9, name: "coprocessor_movefromRt",                          func: _coprocessor_movefrom_rt } }
       10 => { &UserOp { index:  10, name: "coprocessor_movefromRt2",                         func: _coprocessor_movefrom_rt2 } }
       11 => { &UserOp { index:  11, name: "coprocessor_movefrom2",                           func: _coprocessor_movefrom2 } }
       12 => { &UserOp { index:  12, name: "coprocessor_store",                               func: _coprocessor_store } }
       13 => { &UserOp { index:  13, name: "coprocessor_store2",                              func: _coprocessor_store2 } }
       14 => { &UserOp { index:  14, name: "coprocessor_storelong",                           func: _coprocessor_storelong } }
       15 => { &UserOp { index:  15, name: "coprocessor_storelong2",                          func: _coprocessor_storelong2 } }
       16 => { &UserOp { index:  16, name: "software_interrupt",                              func: _software_interrupt } }
       17 => { &UserOp { index:  17, name: "software_bkpt",                                   func: _software_bkpt } }
       18 => { &UserOp { index:  18, name: "software_udf",                                    func: _software_udf } }
       19 => { &UserOp { index:  19, name: "software_hlt",                                    func: _software_hlt } }
       20 => { &UserOp { index:  20, name: "software_hvc",                                    func: _software_hvc } }
       21 => { &UserOp { index:  21, name: "software_smc",                                    func: _software_smc } }
       22 => { &UserOp { index:  22, name: "setUserMode",                                     func: _set_user_mode } }
       23 => { &UserOp { index:  23, name: "setFIQMode",                                      func: _set_fiq_mode } }
       24 => { &UserOp { index:  24, name: "setIRQMode",                                      func: _set_irq_mode } }
       25 => { &UserOp { index:  25, name: "setSupervisorMode",                               func: _set_supervisor_mode } }
       26 => { &UserOp { index:  26, name: "setMonitorMode",                                  func: _set_monitor_mode } }
       27 => { &UserOp { index:  27, name: "setAbortMode",                                    func: _set_abort_mode } }
       28 => { &UserOp { index:  28, name: "setUndefinedMode",                                func: _set_undefined_mode } }
       29 => { &UserOp { index:  29, name: "setSystemMode",                                   func: _set_system_mode } }
       30 => { &UserOp { index:  30, name: "enableIRQinterrupts",                             func: _enable_irq_interrupts } }
       31 => { &UserOp { index:  31, name: "enableFIQinterrupts",                             func: _enable_fiq_interrupts } }
       32 => { &UserOp { index:  32, name: "enableDataAbortInterrupts",                       func: _enable_dataabort_interrupts } }
       33 => { &UserOp { index:  33, name: "disableIRQinterrupts",                            func: _disable_irq_interrupts } }
       34 => { &UserOp { index:  34, name: "disableFIQinterrupts",                            func: _disable_fiq_interrupts } }
       35 => { &UserOp { index:  35, name: "isFIQinterruptsEnabled",                          func: _is_fiq_interrupts_enabled } }
       36 => { &UserOp { index:  36, name: "isIRQinterruptsEnabled",                          func: _is_irq_interrupts_enabled } }
       37 => { &UserOp { index:  37, name: "disableDataAbortInterrupts",                      func: _disable_dataabort_interrupts } }
       38 => { &UserOp { index:  38, name: "hasExclusiveAccess",                              func: _has_exclusive_access } }
       39 => { &UserOp { index:  39, name: "isCurrentModePrivileged",                         func: _is_current_mode_privileged } }
       40 => { &UserOp { index:  40, name: "setThreadModePrivileged",                         func: _set_thread_mode_privileged } }
       41 => { &UserOp { index:  41, name: "isThreadMode",                                    func: _is_thread_mode } }
       42 => { &UserOp { index:  42, name: "jazelle_branch",                                  func: _jazelle_branch } }
       43 => { &UserOp { index:  43, name: "ClearExclusiveLocal",                             func: _clear_exclusive_local } }
       44 => { &UserOp { index:  44, name: "HintDebug",                                       func: _hint_debug } }
       45 => { &UserOp { index:  45, name: "DataMemoryBarrier",                               func: _data_memory_barrier } }
       46 => { &UserOp { index:  46, name: "DataSynchronizationBarrier",                      func: _data_synchronization_barrier } }
       47 => { &UserOp { index:  47, name: "secureMonitorfunc",                               func: _secure_monitor_func } }
       48 => { &UserOp { index:  48, name: "WaitForEvent",                                    func: _wait_for_event } }
       49 => { &UserOp { index:  49, name: "WaitForInterrupt",                                func: _wait_for_interrupt } }
       50 => { &UserOp { index:  50, name: "HintYield",                                       func: _hint_yield } }
       51 => { &UserOp { index:  51, name: "InstructionSynchronizationBarrier",               func: _instruction_synchronization_barrier } }
       52 => { &UserOp { index:  52, name: "HintPreloadData",                                 func: _hint_preload_data } }
       53 => { &UserOp { index:  53, name: "HintPreloadDataForWrite",                         func: _hint_preload_data_for_write } }
       54 => { &UserOp { index:  54, name: "HintPreloadInstruction",                          func: _hint_preload_instruction } }
       55 => { &UserOp { index:  55, name: "SignedSaturate",                                  func: _signed_saturate } }
       56 => { &UserOp { index:  56, name: "SignedDoesSaturate",                              func: _signed_does_saturate } }
       57 => { &UserOp { index:  57, name: "UnsignedSaturate",                                func: _unsigned_saturate } }
       58 => { &UserOp { index:  58, name: "UnsignedDoesSaturate",                            func: _unsigned_does_saturate } }
       59 => { &UserOp { index:  59, name: "Absolute",                                        func: _absolute } }
       60 => { &UserOp { index:  60, name: "ReverseBitOrder",                                 func: _reverse_bit_order } }
       61 => { &UserOp { index:  61, name: "SendEvent",                                       func: _send_event } }
       62 => { &UserOp { index:  62, name: "setEndianState",                                  func: _set_endian_state } }
       63 => { &UserOp { index:  63, name: "coproc_moveto_Main_ID",                           func: _coproc_moveto_main_id } }
       64 => { &UserOp { index:  64, name: "coproc_moveto_Cache_Type",                        func: _coproc_moveto_cache_type } }
       65 => { &UserOp { index:  65, name: "coproc_moveto_TCM_Status",                        func: _coproc_moveto_tcm_status } }
       66 => { &UserOp { index:  66, name: "coproc_moveto_TLB_Type",                          func: _coproc_moveto_tlb_type } }
       67 => { &UserOp { index:  67, name: "coproc_moveto_Control",                           func: _coproc_moveto_control } }
       68 => { &UserOp { index:  68, name: "coproc_moveto_Auxiliary_Control",                 func: _coproc_moveto_auxiliary_control } }
       69 => { &UserOp { index:  69, name: "coproc_moveto_Coprocessor_Access_Control",        func: _coproc_moveto_coprocessor_access_control } }
       70 => { &UserOp { index:  70, name: "coproc_moveto_Secure_Configuration",              func: _coproc_moveto_secure_configuration } }
       71 => { &UserOp { index:  71, name: "coproc_moveto_Secure_Debug_Enable",               func: _coproc_moveto_secure_debug_enable } }
       72 => { &UserOp { index:  72, name: "coproc_moveto_NonSecure_Access_Control",          func: _coproc_moveto_non_secure_access_control } }
       73 => { &UserOp { index:  73, name: "coproc_moveto_Translation_table_base_0",          func: _coproc_moveto_translation_table_base_0 } }
       74 => { &UserOp { index:  74, name: "coproc_moveto_Translation_table_base_1",          func: _coproc_moveto_translation_table_base_1 } }
       75 => { &UserOp { index:  75, name: "coproc_moveto_Translation_table_control",         func: _coproc_moveto_translation_table_control } }
       76 => { &UserOp { index:  76, name: "coproc_moveto_Domain_Access_Control",             func: _coproc_moveto_domain_access_control } }
       77 => { &UserOp { index:  77, name: "coproc_moveto_Data_Fault_Status",                 func: _coproc_moveto_data_fault_status } }
       78 => { &UserOp { index:  78, name: "coproc_moveto_Instruction_Fault_Status",          func: _coproc_moveto_instruction_fault_status } }
       79 => { &UserOp { index:  79, name: "coproc_moveto_Instruction_Fault_Address",         func: _coproc_moveto_instruction_fault_address } }
       80 => { &UserOp { index:  80, name: "coproc_moveto_Fault_Address",                     func: _coproc_moveto_fault_address } }
       81 => { &UserOp { index:  81, name: "coproc_moveto_Instruction_Fault",                 func: _coproc_moveto_instruction_fault } }
       82 => { &UserOp { index:  82, name: "coproc_moveto_Wait_for_interrupt",                func: _coproc_moveto_wait_for_interrupt } }
       83 => { &UserOp { index:  83, name: "coproc_moveto_Invalidate_Entire_Instruction",     func: _coproc_moveto_invalidate_entire_instruction } }
       84 => { &UserOp { index:  84, name: "coproc_moveto_Invalidate_Instruction_Cache_by_MVA", func: _coproc_moveto_invalidate_instruction_cache_by_mva } }
       85 => { &UserOp { index:  85, name: "coproc_moveto_Flush_Prefetch_Buffer",             func: _coproc_moveto_flush_prefetch_buffer } }
       86 | 94 => { &UserOp { index:  86, name: "coproc_moveto_Invalidate_Entire_Data_Cache",      func: _coproc_moveto_invalidate_entire_data_cache } } // duplicated but name has "cache" vs "Cache"
       87 => { &UserOp { index:  87, name: "coproc_moveto_Invalidate_Entire_Data_by_MVA",     func: _coproc_moveto_invalidate_entire_data_by_mva } }
       88 => { &UserOp { index:  88, name: "coproc_moveto_Invalidate_Entire_Data_by_Index",   func: _coproc_moveto_invalidate_entire_data_by_index } }
       89 => { &UserOp { index:  89, name: "coproc_moveto_Clean_Entire_Data_Cache",           func: _coproc_moveto_clean_entire_data_cache } }
       90 => { &UserOp { index:  90, name: "coproc_moveto_Clean_Data_Cache_by_MVA",           func: _coproc_moveto_clean_data_cache_by_mva } }
       91 => { &UserOp { index:  91, name: "coproc_moveto_Clean_Data_Cache_by_Index",         func: _coproc_moveto_clean_data_cache_by_index } }
       92 => { &UserOp { index:  92, name: "coproc_moveto_Data_Synchronization",              func: _coproc_moveto_data_synchronization } }
       93 => { &UserOp { index:  93, name: "coproc_moveto_Data_Memory_Barrier",               func: _coproc_moveto_data_memory_barrier } }
       95 => { &UserOp { index:  95, name: "coproc_moveto_Invalidate_Data_Cache_by_MVA",      func: _coproc_moveto_invalidate_data_cache_by_mva } }
       96 => { &UserOp { index:  96, name: "coproc_moveto_Invalidate_unified_TLB_unlocked",   func: _coproc_moveto_invalidate_unified_tlb_unlocked } }
       97 => { &UserOp { index:  97, name: "coproc_moveto_Invalidate_unified_TLB_by_MVA",     func: _coproc_moveto_invalidate_unified_tlb_by_mva } }
       98 => { &UserOp { index:  98, name: "coproc_moveto_Invalidate_unified_TLB_by_ASID_match", func: _coproc_moveto_invalidate_unified_tlb_by_asid_match } }
       99 => { &UserOp { index:  99, name: "coproc_moveto_FCSE_PID",                          func: _coproc_moveto_fcse_pid } }
      100 => { &UserOp { index: 100, name: "coproc_moveto_Context_ID",                        func: _coproc_moveto_context_id } }
      101 => { &UserOp { index: 101, name: "coproc_moveto_User_RW_Thread_and_Process_ID",     func: _coproc_moveto_user_rw_thread_and_process_id } }
      102 => { &UserOp { index: 102, name: "coproc_moveto_User_R_Thread_and_Process_ID",      func: _coproc_moveto_user_r_thread_and_process_id } }
      103 => { &UserOp { index: 103, name: "coproc_moveto_Privileged_only_Thread_and_Process_ID", func: _coproc_moveto_privileged_only_thread_and_process_id } }
      104 => { &UserOp { index: 104, name: "coproc_moveto_Peripherial_Port_Memory_Remap",     func: _coproc_moveto_peripherial_port_memory_remap } }
      105 => { &UserOp { index: 105, name: "coproc_moveto_Feature_Identification",            func: _coproc_moveto_feature_identification } }
      106 => { &UserOp { index: 106, name: "coproc_moveto_ISA_Feature_Identification",        func: _coproc_moveto_isa_feature_identification } }
      107 => { &UserOp { index: 107, name: "coproc_moveto_Peripheral_Port_Memory_Remap",      func: _coproc_moveto_peripheral_port_memory_remap } }
      108 => { &UserOp { index: 108, name: "coproc_moveto_Control_registers",                 func: _coproc_moveto_control_registers } }
      109 => { &UserOp { index: 109, name: "coproc_moveto_Security_world_control",            func: _coproc_moveto_security_world_control } }
      110 => { &UserOp { index: 110, name: "coproc_moveto_Translation_table",                 func: _coproc_moveto_translation_table } }
      111 => { &UserOp { index: 111, name: "coproc_moveto_Instruction_cache",                 func: _coproc_moveto_instruction_cache } }
      112 => { &UserOp { index: 112, name: "coproc_moveto_Data_cache_operations",             func: _coproc_moveto_data_cache_operations } }
      113 => { &UserOp { index: 113, name: "coproc_moveto_Identification_registers",          func: _coproc_moveto_identification_registers } }
      114 => { &UserOp { index: 114, name: "coproc_moveto_Peripheral_System",                 func: _coproc_moveto_peripheral_system } }
      115 => { &UserOp { index: 115, name: "coproc_movefrom_Main_ID",                         func: _coproc_movefrom_main_id } }
      116 => { &UserOp { index: 116, name: "coproc_movefrom_Cache_Type",                      func: _coproc_movefrom_cache_type } }
      117 => { &UserOp { index: 117, name: "coproc_movefrom_TCM_Status",                      func: _coproc_movefrom_tcm_status } }
      118 => { &UserOp { index: 118, name: "coproc_movefrom_TLB_Type",                        func: _coproc_movefrom_tlb_type } }
      119 => { &UserOp { index: 119, name: "coproc_movefrom_Control",                         func: _coproc_movefrom_control } }
      120 => { &UserOp { index: 120, name: "coproc_movefrom_Auxiliary_Control",               func: _coproc_movefrom_auxiliary_control } }
      121 => { &UserOp { index: 121, name: "coproc_movefrom_Coprocessor_Access_Control",      func: _coproc_movefrom_coprocessor_access_control } }
      122 => { &UserOp { index: 122, name: "coproc_movefrom_Secure_Configuration",            func: _coproc_movefrom_secure_configuration } }
      123 => { &UserOp { index: 123, name: "coproc_movefrom_Secure_Debug_Enable",             func: _coproc_movefrom_secure_debug_enable } }
      124 => { &UserOp { index: 124, name: "coproc_movefrom_NonSecure_Access_Control",        func: _coproc_movefrom_non_secure_access_control } }
      125 => { &UserOp { index: 125, name: "coproc_movefrom_Translation_table_base_0",        func: _coproc_movefrom_translation_table_base_0 } }
      126 => { &UserOp { index: 126, name: "coproc_movefrom_Translation_table_base_1",        func: _coproc_movefrom_translation_table_base_1 } }
      127 => { &UserOp { index: 127, name: "coproc_movefrom_Translation_table_control",       func: _coproc_movefrom_translation_table_control } }
      128 => { &UserOp { index: 128, name: "coproc_movefrom_Domain_Access_Control",           func: _coproc_movefrom_domain_access_control } }
      129 => { &UserOp { index: 129, name: "coproc_movefrom_Data_Fault_Status",               func: _coproc_movefrom_data_fault_status } }
      130 => { &UserOp { index: 130, name: "coproc_movefrom_Instruction_Fault",               func: _coproc_movefrom_instruction_fault } }
      131 => { &UserOp { index: 131, name: "coproc_movefrom_Fault_Address",                   func: _coproc_movefrom_fault_address } }
      132 => { &UserOp { index: 132, name: "coproc_movefrom_Instruction_Fault_Status",        func: _coproc_movefrom_instruction_fault_status } }
      133 => { &UserOp { index: 133, name: "coproc_movefrom_Instruction_Fault_Address",       func: _coproc_movefrom_instruction_fault_address } }
      134 => { &UserOp { index: 134, name: "coproc_movefrom_Wait_for_interrupt",              func: _coproc_movefrom_wait_for_interrupt } }
      135 => { &UserOp { index: 135, name: "coproc_movefrom_Invalidate_Entire_Instruction",   func: _coproc_movefrom_invalidate_entire_instruction } }
      136 => { &UserOp { index: 136, name: "coproc_movefrom_Invalidate_Instruction_Cache_by_MVA", func: _coproc_movefrom_invalidate_instruction_cache_by_mva } }
      137 => { &UserOp { index: 137, name: "coproc_movefrom_Flush_Prefetch_Buffer",           func: _coproc_movefrom_flush_prefetch_buffer } }
      138 | 146 => { &UserOp { index: 138, name: "coproc_movefrom_Invalidate_Entire_Data_cache",    func: _coproc_movefrom_invalidate_entire_data_cache } } // duplicated but name has "cache" vs "Cache"
      139 => { &UserOp { index: 139, name: "coproc_movefrom_Invalidate_Entire_Data_by_MVA",   func: _coproc_movefrom_invalidate_entire_data_by_mva } }
      140 => { &UserOp { index: 140, name: "coproc_movefrom_Invalidate_Entire_Data_by_Index", func: _coproc_movefrom_invalidate_entire_data_by_index } }
      141 => { &UserOp { index: 141, name: "coproc_movefrom_Clean_Entire_Data_Cache",         func: _coproc_movefrom_clean_entire_data_cache } }
      142 => { &UserOp { index: 142, name: "coproc_movefrom_Clean_Data_Cache_by_MVA",         func: _coproc_movefrom_clean_data_cache_by_mva } }
      143 => { &UserOp { index: 143, name: "coproc_movefrom_Clean_Data_Cache_by_Index",       func: _coproc_movefrom_clean_data_cache_by_index } }
      144 => { &UserOp { index: 144, name: "coproc_movefrom_Data_Synchronization",            func: _coproc_movefrom_data_synchronization } }
      145 => { &UserOp { index: 145, name: "coproc_movefrom_Data_Memory_Barrier",             func: _coproc_movefrom_data_memory_barrier } }
      147 => { &UserOp { index: 147, name: "coproc_movefrom_Invalidate_Data_Cache_by_MVA",    func: _coproc_movefrom_invalidate_data_cache_by_mva } }
      148 => { &UserOp { index: 148, name: "coproc_movefrom_Invalidate_unified_TLB_unlocked", func: _coproc_movefrom_invalidate_unified_tlb_unlocked } }
      149 => { &UserOp { index: 149, name: "coproc_movefrom_Invalidate_unified_TLB_by_MVA",   func: _coproc_movefrom_invalidate_unified_tlb_by_mva } }
      150 => { &UserOp { index: 150, name: "coproc_movefrom_Invalidate_unified_TLB_by_ASID_match", func: _coproc_movefrom_invalidate_unified_tlb_by_asid_match } }
      151 => { &UserOp { index: 151, name: "coproc_movefrom_FCSE_PID",                        func: _coproc_movefrom_fcse_pid } }
      152 => { &UserOp { index: 152, name: "coproc_movefrom_Context_ID",                      func: _coproc_movefrom_context_id } }
      153 => { &UserOp { index: 153, name: "coproc_movefrom_User_RW_Thread_and_Process_ID",   func: _coproc_movefrom_user_rw_thread_and_process_id } }
      154 => { &UserOp { index: 154, name: "coproc_movefrom_User_R_Thread_and_Process_ID",    func: _coproc_movefrom_user_r_thread_and_process_id } }
      155 => { &UserOp { index: 155, name: "coproc_movefrom_Privileged_only_Thread_and_Process_ID", func: _coproc_movefrom_privileged_only_thread_and_process_id } }
      156 => { &UserOp { index: 156, name: "coproc_movefrom_Peripherial_Port_Memory_Remap",   func: _coproc_movefrom_peripherial_port_memory_remap } }
      157 => { &UserOp { index: 157, name: "coproc_movefrom_Feature_Identification",          func: _coproc_movefrom_feature_identification } }
      158 => { &UserOp { index: 158, name: "coproc_movefrom_ISA_Feature_Identification",      func: _coproc_movefrom_isa_feature_identification } }
      159 => { &UserOp { index: 159, name: "coproc_movefrom_Peripheral_Port_Memory_Remap",    func: _coproc_movefrom_peripheral_port_memory_remap } }
      160 => { &UserOp { index: 160, name: "coproc_movefrom_Control_registers",               func: _coproc_movefrom_control_registers } }
      161 => { &UserOp { index: 161, name: "coproc_movefrom_Security_world_control",          func: _coproc_movefrom_security_world_control } }
      162 => { &UserOp { index: 162, name: "coproc_movefrom_Translation_table",               func: _coproc_movefrom_translation_table } }
      163 => { &UserOp { index: 163, name: "coproc_movefrom_Instruction_cache",               func: _coproc_movefrom_instruction_cache } }
      164 => { &UserOp { index: 164, name: "coproc_movefrom_Data_cache_operations",           func: _coproc_movefrom_data_cache_operations } }
      165 => { &UserOp { index: 165, name: "coproc_movefrom_Identification_registers",        func: _coproc_movefrom_identification_registers } }
      166 => { &UserOp { index: 166, name: "coproc_movefrom_Peripheral_System",               func: _coproc_movefrom_peripheral_system } }
      167 => { &UserOp { index: 167, name: "VFPExpandImmediate",                              func: _vfp_expand_immediate } }
      168 => { &UserOp { index: 168, name: "SIMDExpandImmediate",                             func: _simd_expand_immediate } }
      169 => { &UserOp { index: 169, name: "VectorAbsoluteDifferenceAndAccumulate",           func: _vector_absolute_difference_and_accumulate } }
      170 => { &UserOp { index: 170, name: "VectorAbsoluteDifference",                        func: _vector_absolute_difference } }
      171 => { &UserOp { index: 171, name: "FloatVectorAbsoluteDifference",                   func: _float_vector_absolute_difference } }
      172 => { &UserOp { index: 172, name: "VectorAbsolute",                                  func: _vector_absolute } }
      173 => { &UserOp { index: 173, name: "FloatVectorAbsolute",                             func: _float_vector_absolute } }
      174 => { &UserOp { index: 174, name: "FloatCompareGE",                                  func: _float_compare_ge } }
      175 => { &UserOp { index: 175, name: "FloatCompareGT",                                  func: _float_compare_gt } }
      176 => { &UserOp { index: 176, name: "VectorAdd",                                       func: _vector_add } }
      177 => { &UserOp { index: 177, name: "VectorSub",                                       func: _vector_sub } }
      178 => { &UserOp { index: 178, name: "FloatVectorAdd",                                  func: _float_vector_add } }
      179 => { &UserOp { index: 179, name: "VectorPairwiseAdd",                               func: _vector_pairwise_add } }
      180 => { &UserOp { index: 180, name: "VectorPairwiseMin",                               func: _vector_pairwise_min } }
      181 => { &UserOp { index: 181, name: "VectorPairwiseMax",                               func: _vector_pairwise_max } }
      182 => { &UserOp { index: 182, name: "FloatVectorPairwiseAdd",                          func: _float_vector_pairwise_add } }
      183 => { &UserOp { index: 183, name: "FloatVectorPairwiseMin",                          func: _float_vector_pairwise_min } }
      184 => { &UserOp { index: 184, name: "FloatVectorPairwiseMax",                          func: _float_vector_pairwise_max } }
      185 => { &UserOp { index: 185, name: "VectorPairwiseAddLong",                           func: _vector_pairwise_add_long } }
      186 => { &UserOp { index: 186, name: "VectorPairwiseAddAccumulateLong",                 func: _vector_pairwise_add_accumulate_long } }
      187 => { &UserOp { index: 187, name: "VectorAddReturnHigh",                             func: _vector_add_return_high } }
      188 => { &UserOp { index: 188, name: "VectorBitwiseInsertIfFalse",                      func: _vector_bitwise_insert_if_false } }
      189 => { &UserOp { index: 189, name: "VectorBitwiseInsertIfTrue",                       func: _vector_bitwise_insert_if_true } }
      190 => { &UserOp { index: 190, name: "VectorBitwiseSelect",                             func: _vector_bitwise_select } }
      191 => { &UserOp { index: 191, name: "VectorCompareEqual",                              func: _vector_compare_equal } }
      192 => { &UserOp { index: 192, name: "FloatVectorCompareEqual",                         func: _float_vector_compare_equal } }
      193 => { &UserOp { index: 193, name: "VectorCompareGreaterThanOrEqual",                 func: _vector_compare_greater_than_or_equal } }
      194 => { &UserOp { index: 194, name: "FloatVectorCompareGreaterThanOrEqual",            func: _float_vector_compare_greater_than_or_equal } }
      195 => { &UserOp { index: 195, name: "VectorCompareGreaterThan",                        func: _vector_compare_greater_than } }
      196 => { &UserOp { index: 196, name: "FloatVectorCompareGreaterThan",                   func: _float_vector_compare_greater_than } }
      197 => { &UserOp { index: 197, name: "VectorCountLeadingSignBits",                      func: _vector_count_leading_sign_bits } }
      198 => { &UserOp { index: 198, name: "VectorCountLeadingZeros",                         func: _vector_count_leading_zeros } }
      199 => { &UserOp { index: 199, name: "VectorCountOneBits",                              func: _vector_count_one_bits } }
      200 => { &UserOp { index: 200, name: "VectorFloatToSigned",                             func: _vector_float_to_signed } }
      201 => { &UserOp { index: 201, name: "VectorFloatToUnsigned",                           func: _vector_float_to_unsigned } }
      202 => { &UserOp { index: 202, name: "VectorSignedToFloat",                             func: _vector_signed_to_float } }
      203 => { &UserOp { index: 203, name: "VectorUnsignedToFloat",                           func: _vector_unsigned_to_float } }
      204 => { &UserOp { index: 204, name: "VectorFloatToSignedFixed",                        func: _vector_float_to_signed_fixed } }
      205 => { &UserOp { index: 205, name: "VectorFloatToUnsignedFixed",                      func: _vector_float_to_unsigned_fixed } }
      206 => { &UserOp { index: 206, name: "VectorSignedFixedToFloat",                        func: _vector_signed_fixed_to_float } }
      207 => { &UserOp { index: 207, name: "VectorUnsignedFixedToFloat",                      func: _vector_unsigned_fixed_to_float } }
      208 => { &UserOp { index: 208, name: "VectorFloatDoubleToSingle",                       func: _vector_float_double_to_single } }
      209 => { &UserOp { index: 209, name: "VectorFloatSingleToDouble",                       func: _vector_float_single_to_double } }
      210 => { &UserOp { index: 210, name: "VectorFloatSingleToHalf",                         func: _vector_float_single_to_half } }
      211 => { &UserOp { index: 211, name: "VectorFloatHalfToSingle",                         func: _vector_float_half_to_single } }
      212 => { &UserOp { index: 212, name: "VectorHalvingAdd",                                func: _vector_halving_add } }
      213 => { &UserOp { index: 213, name: "VectorHalvingSubtract",                           func: _vector_halving_subtract } }
      214 => { &UserOp { index: 214, name: "VectorRoundHalvingAdd",                           func: _vector_round_halving_add } }
      215 => { &UserOp { index: 215, name: "VectorRoundAddAndNarrow",                         func: _vector_round_add_and_narrow } }
      216 => { &UserOp { index: 216, name: "VectorMin",                                       func: _vector_min } }
      217 => { &UserOp { index: 217, name: "VectorMax",                                       func: _vector_max } }
      218 => { &UserOp { index: 218, name: "FloatVectorMin",                                  func: _float_vector_min } }
      219 => { &UserOp { index: 219, name: "FloatVectorMax",                                  func: _float_vector_max } }
      220 => { &UserOp { index: 220, name: "VectorMultiplyAccumulate",                        func: _vector_multiply_accumulate } }
      221 => { &UserOp { index: 221, name: "VectorMultiplySubtract",                          func: _vector_multiply_subtract } }
      222 => { &UserOp { index: 222, name: "VectorMultiplySubtractLong",                      func: _vector_multiply_subtract_long } }
      223 => { &UserOp { index: 223, name: "VectorDoubleMultiplyHighHalf",                    func: _vector_double_multiply_high_half } }
      224 => { &UserOp { index: 224, name: "VectorRoundDoubleMultiplyHighHalf",               func: _vector_round_double_multiply_high_half } }
      225 => { &UserOp { index: 225, name: "VectorDoubleMultiplyLong",                        func: _vector_double_multiply_long } }
      226 => { &UserOp { index: 226, name: "VectorDoubleMultiplyAccumulateLong",              func: _vector_double_multiply_accumulate_long } }
      227 => { &UserOp { index: 227, name: "VectorDoubleMultiplySubtractLong",                func: _vector_double_multiply_subtract_long } }
      228 => { &UserOp { index: 228, name: "FloatVectorMultiplyAccumulate",                   func: _float_vector_multiply_accumulate } }
      229 => { &UserOp { index: 229, name: "FloatVectorMultiplySubtract",                     func: _float_vector_multiply_subtract } }
      230 => { &UserOp { index: 230, name: "VectorGetElement",                                func: _vector_get_element } }
      231 => { &UserOp { index: 231, name: "VectorSetElement",                                func: _vector_set_element } }
      232 => { &UserOp { index: 232, name: "VectorCopyLong",                                  func: _vector_copy_long } }
      233 => { &UserOp { index: 233, name: "VectorCopyNarrow",                                func: _vector_copy_narrow } }
      234 => { &UserOp { index: 234, name: "FloatVectorMult",                                 func: _float_vector_mult } }
      235 => { &UserOp { index: 235, name: "VectorMultiply",                                  func: _vector_multiply } }
      236 => { &UserOp { index: 236, name: "PolynomialMultiply",                              func: _polynomial_multiply } }
      237 => { &UserOp { index: 237, name: "FloatVectorNeg",                                  func: _float_vector_neg } }
      238 => { &UserOp { index: 238, name: "SatQ",                                            func: _sat_q } }
      239 => { &UserOp { index: 239, name: "SignedSatQ",                                      func: _signed_sat_q } }
      240 => { &UserOp { index: 240, name: "VectorReciprocalEstimate",                        func: _vector_reciprocal_estimate } }
      241 => { &UserOp { index: 241, name: "VectorReciprocalStep",                            func: _vector_reciprocal_step } }
      242 => { &UserOp { index: 242, name: "vrev",                                            func: _vrev } }
      243 => { &UserOp { index: 243, name: "VectorShiftLeft",                                 func: _vector_shift_left } }
      244 => { &UserOp { index: 244, name: "VectorRoundShiftLeft",                            func: _vector_round_shift_left } }
      245 => { &UserOp { index: 245, name: "VectorShiftRight",                                func: _vector_shift_right } }
      246 => { &UserOp { index: 246, name: "VectorShiftLeftInsert",                           func: _vector_shift_left_insert } }
      247 => { &UserOp { index: 247, name: "VectorShiftRightInsert",                          func: _vector_shift_right_insert } }
      248 => { &UserOp { index: 248, name: "VectorShiftRightNarrow",                          func: _vector_shift_right_narrow } }
      249 => { &UserOp { index: 249, name: "VectorShiftRightAccumulate",                      func: _vector_shift_right_accumulate } }
      250 => { &UserOp { index: 250, name: "VectorRoundShiftRight",                           func: _vector_round_shift_right } }
      251 => { &UserOp { index: 251, name: "VectorRoundShiftRightNarrow",                     func: _vector_round_shift_right_narrow } }
      252 => { &UserOp { index: 252, name: "VectorRoundShiftRightAccumulate",                 func: _vector_round_shift_right_accumulate } }
      253 => { &UserOp { index: 253, name: "VectorShiftLongLeft",                             func: _vector_shift_long_left } }
      254 => { &UserOp { index: 254, name: "VectorShiftNarrowRight",                          func: _vector_shift_narrow_right } }
      255 => { &UserOp { index: 255, name: "VectorReciprocalSquareRootEstimate",              func: _vector_reciprocal_square_root_estimate } }
      256 => { &UserOp { index: 256, name: "VectorReciprocalSquareRootStep",                  func: _vector_reciprocal_square_root_step } }
      257 => { &UserOp { index: 257, name: "FloatVectorSub",                                  func: _float_vector_sub } }
      258 => { &UserOp { index: 258, name: "VectorSubAndNarrow",                              func: _vector_sub_and_narrow } }
      259 => { &UserOp { index: 259, name: "VectorTableLookup",                               func: _vector_table_lookup } }
      260 => { &UserOp { index: 260, name: "VectorTest",                                      func: _vector_test } }
      261 => { &UserOp { index: 261, name: "VectorTranspose",                                 func: _vector_transpose } }
      262 => { &UserOp { index: 262, name: "VectorUnzip",                                     func: _vector_unzip } }
      263 => { &UserOp { index: 263, name: "VectorZip",                                       func: _vector_zip } }
      264 => { &UserOp { index: 264, name: "SG",                                              func: _sg } }
      265 => { &UserOp { index: 265, name: "TT",                                              func: _tt } }
      266 => { &UserOp { index: 266, name: "TTA",                                             func: _tta } }
      267 => { &UserOp { index: 267, name: "TTAT",                                            func: _ttat } }
      268 => { &UserOp { index: 268, name: "TTT",                                             func: _ttt } }
      269 => { &UserOp { index: 269, name: "IndexCheck",                                      func: _index_check } }
      270 => { &UserOp { index: 270, name: "ExclusiveAccess",                                 func: _exclusive_access } }
      271 => { &UserOp { index: 271, name: "getMainStackPointer",                             func: _get_main_stack_pointer } }
      272 => { &UserOp { index: 272, name: "getProcessStackPointer",                          func: _get_process_stack_pointer } }
      273 => { &UserOp { index: 273, name: "getBasePriority",                                 func: _get_base_priority } }
      274 => { &UserOp { index: 274, name: "getCurrentExceptionNumber",                       func: _get_current_exception_number } }
      275 => { &UserOp { index: 275, name: "isThreadModePrivileged",                          func: _is_thread_mode_privileged } }
      276 => { &UserOp { index: 276, name: "isUsingMainStack",                                func: _is_using_main_stack } }
      277 => { &UserOp { index: 277, name: "setMainStackPointer",                             func: _set_main_stack_pointer } }
      278 => { &UserOp { index: 278, name: "setProcessStackPointer",                          func: _set_process_stack_pointer } }
      279 => { &UserOp { index: 279, name: "setBasePriority",                                 func: _set_base_priority } }
      280 => { &UserOp { index: 280, name: "setStackMode",                                    func: _set_stack_mode } }

        _ => { panic!("invalid userop: {}", index) }
    }
}

/// implements the CLZ instruction.
/// 
/// probably defined as a userop in order to take advantage of 
/// host hardware implementation
/// 
/// inputs:
/// - value to count leading zeroes in
/// output:
/// - number of leading zeroes
fn _count_leading_zeroes(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.len() == 1, "count_leading_zeros expects exactly 1 input!");
    assert!(output.is_some(), "count_leading_zeros expects an output");
    let input0 = &inputs[0];
    let dst = output.unwrap();
    let in0_bv = this._read_vnd(input0)?;
    let result = BitVec::from_u32(
        in0_bv.leading_zeros(),
        dst.bits(),
    );
    this._write_vnd(dst, &result)?;
    Ok(None)
}

/// implements the SVC instruction in armv7m
/// 
/// generates an SVfunc exception
/// 
/// inputs:
/// - none
/// output:
/// - none
fn _software_interrupt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.is_empty(), "software_interrupt expects no inputs");
    assert!(output.is_none(), "software_interrupt has not output");
    let excp = ExceptionType::SVCall;
    let evt = Event::ExceptionSetActive(excp, true);
    this.events.push_back(evt);
    Ok(None)
}

fn _software_bkpt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _software_udf(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _software_hlt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _software_hvc(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _software_smc(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _set_user_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _set_fiq_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _set_irq_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _set_supervisor_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _set_monitor_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _set_abort_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _set_undefined_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _set_system_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// enables irq interrupts.
/// called for "cpsie i" instruction.
/// should clear PRIMASK, per B1.4.3.
/// see B5.2.1 for CPS instruction
/// 
/// note: also appears to be called in the sleigh definition
/// for the "msr primask, <in>" instruction, but it takes in
/// a parameter, which may need to be handled specially.
/// ARMTHUMBinstructions.sinc
/// 
/// inputs:
/// - none
/// output:
/// - none
#[instrument]
fn _enable_irq_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.is_empty(), "enable_irq_interrupts expects no inputs");
    assert!(output.is_none(), "enable_irq_interrupts has no output");
    info!("{}", _lookup_userop(index).name);
    this.primask.set_pm(false);
    Ok(None)
}

/// enables fiq interrupts.
/// called for "cpsie f" instruction.
/// should clear FAULTMASK, per B1.4.3.
/// see B5.2.1 for CPS instruction
/// 
/// note: also appears to be called in the sleigh definition
/// for the "msr faultmask, <in>" instruction, but it takes in
/// a parameter, which may need to be handled specially.
/// ARMTHUMBinstructions.sinc
/// 
/// inputs:
/// - none
/// output:
/// - none
#[instrument]
fn _enable_fiq_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.is_empty(), "enable_fiq_interrupts expects no inputs");
    assert!(output.is_none(), "enable_fiq_interrupts has no output");
    info!("{}", _lookup_userop(index).name);
    this.faultmask.set_fm(false);
    Ok(None)
}

fn _enable_dataabort_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// disables irq interrupts.
/// called for "cpsid i" instruction.
/// should set PRIMASK, per B1.4.3.
/// see B5.2.1 for CPS instruction
/// 
/// inputs: 
/// - none
/// output:
/// - none
#[instrument]
fn _disable_irq_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.is_empty(), "disable_irq_interrupts expects no inputs");
    assert!(output.is_none(), "disable_irq_interrupts has not output");
    info!("{}", _lookup_userop(index).name);
    this.primask.set_pm(true);
    Ok(None)
}

/// disables fiq interrupts.
/// called for "cpsid f" instruction.
/// should set FAULTMASK, per B1.4.3.
/// see B5.2.1 for CPS instruction
/// 
/// inputs:
/// - none
/// output:
/// - none
#[instrument]
fn _disable_fiq_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.is_empty(), "disable_fiq_interrupts expects no inputs");
    assert!(output.is_none(), "disable_fiq_interrupts has not output");
    info!("{}", _lookup_userop(index).name);
    this.faultmask.set_fm(true);
    Ok(None)
}

fn _is_fiq_interrupts_enabled(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _is_irq_interrupts_enabled(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _disable_dataabort_interrupts(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// checks if executing processor has exclusive access
/// to the memory addressed.
/// used in sleigh definitions of "strex" instructions.
/// see A7.7.167 for STREX instruction.
/// see A7-184 for info about memory accesses.
/// see A3.4 for info about arch support for synchronization and semaphores.
/// 
/// inputs:
/// - memory address to check access rights for
/// output:
/// - 1 if executing processor has exclusive access, 0 otherwise.
#[instrument]
fn _has_exclusive_access(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.len() == 1, "has_exclusive_access expects exactly 1 input");
    assert!(output.is_some(), "has_exclusive_access has an output");
    warn!("has_exclusive_access() is currently a stub that always returns 1");
    let out = output.unwrap();
    let bv = bool2bv(true);
    this._write_vnd(out, &bv)?;
    Ok(None)
}

/// checks if current execution mode is privileged, called 
/// when executing MRS and MSR instructions.
/// should be identical to "CurrentModeIsPrivileged()" defined 
/// in B1.3.1.
/// 
/// inputs:
/// - none
/// output:
/// - 1 if current mode is privileged, 0 otherwise
#[instrument]
fn _is_current_mode_privileged(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.is_empty(), "is_current_mode_privileged expects no inputs");
    assert!(output.is_some(), "is_current_mode_privileged has an output");
    let out = output.unwrap();
    let bv = bool2bv(this.current_mode_is_privileged());
    this._write_vnd(out, &bv)?;
    Ok(None)
}

/// sets the execution privilege of thread mode to privileged.
/// used in "msr  control, <Rn>" on write to special-purpose
/// CONTROL (B1.4.4)
/// 
/// inputs:
/// - 0 if set to unprivileged (nPRIV=1), 1 if set to privileged (nPRIV=0)
/// output:
/// - none
fn _set_thread_mode_privileged(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.len() == 1, "set_thread_mode_privileged expects exactly 1 input");
    assert!(output.is_none(), "set_thread_mode_privileged has no output");
    let val = this._read_vnd(&inputs[0])?.bit(0);
    this.control.set_npriv(!val);
    Ok(None)
}

/// checks if processor currently executing in thread mode
/// used in "msr  control, <Rn>" on write to special-purpose
/// CONTROL (B1.4.4)
/// 
/// inputs:
/// - none
/// output:
/// - 1 if processor in thread mode, 0 otherwise
fn _is_thread_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.is_empty(), "is_thread_mode expects no inputs");
    assert!(output.is_some(), "is_thread_mode has an output");
    let out = output.unwrap();
    let bv = bool2bv(this.mode == Mode::Thread);
    this._write_vnd(out, &bv)?;
    Ok(None)
}

fn _jazelle_branch(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _clear_exclusive_local(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _hint_debug(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

/// implementation of DMB instruction.
/// (see DMB instruction A7.7.33)
/// (see Memory barriers in A3.7.3)
#[instrument]
fn _data_memory_barrier(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.len() == 1, "data_memory_barrier expects exactly 1 argument");
    assert!(output.is_none(), "data_memory_barrier has no output");
    warn!("data_memory_barrier() is currently a stub that does nothing.");
    let option_val = this._read_vnd(&inputs[0])?.to_u8().unwrap();
    // see A7.7.33, option_val = 0b1111 for SY DMB operation, and others are reserved,
    // but reserved instructions also behave as SY (software shouldn't rely on this
    // behavior)
    if option_val != 0xf {
        warn!("DMB instruction expects option 0xf, got {option_val:#x}");
    }
    Ok(None)
}

/// implementation of DSB instruction.
/// (see DSB instruction A7.7.34)
/// (see Memory barriers in A3.7.3)
#[instrument]
fn _data_synchronization_barrier(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    assert!(inputs.len() == 1, "data_synchronization_barrier expects exactly 1 argument");
    assert!(output.is_none(), "data_synchronization_barrier has no output");
    warn!("data_synchronization_barrier is currently a stub that does nothing.");
    let option_val = this._read_vnd(&inputs[0])?.to_u8().unwrap();
    // see A7.7.34, option_val = 0b1111 for SY DSB operation, and others are reserved,
    // but reserved instructions also behave as SY (software shouldn't rely on this
    // behavior)
    if option_val != 0xf {
        warn!("DSB instruction expects option 0xf, got {option_val:#x}");
    }
    Ok(None)
}

fn _secure_monitor_func(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

/// implementation of WFE instruction.
/// (see WFE instruction A7.7.261)
/// 
/// put processor or thread in suspension until an event occurs.
/// (see Wait For Event and Send Event B1.5.18)
/// 
/// inputs:
/// - none
/// output:
/// - none
fn _wait_for_event(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    if this.event.0 {
        this.event.0 = false;
    } else {
        let evt = Event::SetProcessorStatus(Status::WaitingForEvent);
        this.events.push_back(evt);
    }
    Ok(None)
}

/// implementation of WFI instruction.
/// (see WFI instruction A7.7.262)
/// 
/// put processor in suspension with fast wakeup until wakeup condition
/// (see Wait For Interrupt B1-562)
fn _wait_for_interrupt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    let evt = Event::SetProcessorStatus(Status::WaitingForInterrupt);
    this.events.push_back(evt);
    Ok(None)
}

fn _hint_yield(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

/// implementation of ISB instruction.
/// (see Memory barriers in A3.7.3)
fn _instruction_synchronization_barrier(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _hint_preload_data(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _hint_preload_data_for_write(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _hint_preload_instruction(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _signed_saturate(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _signed_does_saturate(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _unsigned_saturate(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _unsigned_does_saturate(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _absolute(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _reverse_bit_order(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _send_event(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _set_endian_state(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _sg(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _tt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _tta(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _ttat(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _ttt(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

fn _index_check(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// if address has shareable memory attribute, mark the physical
/// address as exclusive access for the executing processor in the
/// global monitor.
/// used in sleigh definitions of "ldrex" instructions.
/// see A7.7.52 for LDREX instruction.
/// see A3.4 for synchronization and semaphores.
/// 
/// size of the tagged block is IMPLEMENTATION DEFINED.
fn _exclusive_access(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// loads current value of the main stack pointer.
/// used in implementation of "MRS  <Rd>, msp".
/// (see SP registers in B1.4.1)
fn _get_main_stack_pointer(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// loads current value of the process stack pointer.
/// used in implementation of "MRS  <Rd>, psp".
/// (see SP registers in B1.4.1)
fn _get_process_stack_pointer(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// loads current value of the BASEPRI register.
/// (see special-purpose mask registers  B1.4.3)
/// (used in a few sleigh implementations involving basepri and basepri_max)
fn _get_base_priority(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// returns the current exception number in IPSR.
/// used in implmentation of "MRS  IPSR" instruction.
/// (see special-purpose Program Status Registers B1.4.2)
fn _get_current_exception_number(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// returns whether current thread mode is privileged.
/// used in implementation of "MRS  CONTROL" instruction.
/// (see special-purpose CONTROL register B1.4.4)
/// 
/// essentially reads !CONTROL.nPRIV
fn _is_thread_mode_privileged(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    let bv = bool2bv(!this.control.npriv());
    this._write_vnd(output.unwrap(), &bv);
    Ok(None)
}

/// returns whether main stack pointer is currently being used.
/// used in implementation of "MRS  CONTROL" instruction.
/// (see special-purpose CONTROL register B1.4.4)
/// 
/// essentially read !CONTROL.SPSEL
fn _is_using_main_stack(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    let bv = bool2bv(!this.control.spsel());
    this._write_vnd(output.unwrap(), &bv);
    Ok(None)
}

/// write to main stack pointer directly.
/// used in implementation of "MSR  MSP, <Rn>" instruction.
/// (remember to update currently used stack pointer if needed)
fn _set_main_stack_pointer(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// write to process stack pointer directly.
/// used in implementation of "MSR  PSP, <Rn>" instruction.
/// (remember to update currently used stack pointer if needed)
fn _set_process_stack_pointer(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    todo!("unsupported userop: {}", _lookup_userop(index).name)
}

/// write to base priority special-purpose register.
/// used in implementation of "MSR  BASEPRI, <Rn>" instruction.
fn _set_base_priority(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    let value = this._read_vnd(&inputs[0])?
        .to_u8().unwrap();
    this.basepri.set_basepri(value);
    Ok(None)
}

/// set current stack mode based in input0.
/// used in implmentation of "MSR  CONTROL, <Rn>" instruction.
/// 
/// NOTE: ARMTHUMBinstruction.sinc definition of this instruction
/// appears to be faulty? not setting value based on Rn...
fn _set_stack_mode(this: &mut Context,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, context::Error> {
    let value = this._read_vnd(&inputs[0])?
        .to_u8().unwrap() != 0;
    this.control.set_spsel(value);
    Ok(None)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_userops() -> Result<(), ()> {
        let indices = &[
            0, 12, 16, 30, 33, 38, 39, 40, 41, 45, 
            46, 48, 49, 51, 270, 271, 272, 273, 274, 
            275, 276, 277, 278, 279, 280,
        ];

        for idx in indices {
            let name = _lookup_userop(*idx).name;
            println!("{idx}: {name}");
        }

        Ok(())
    }
}