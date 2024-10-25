//! mpu.rs
//! 
//! memory protection unit implementation

use derive_more::{From, TryFrom, TryInto};
use bitfield_struct::bitfield;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MPURegType {
    /// mpu type register
    TYPE,
    /// mpu control register
    CTRL,
    /// mpu region number register
    RNR,
    /// mpu region base address register
    /// has alias 1, 2, and 3, (0 = original)
    RBAR(u8),
    /// mpu region attribute and size register
    /// has alias 1, 2, and 3, (0 = original)
    RASR(u8),
}

#[derive(Debug, Clone)]
struct MPURegData {
    pub offset: usize,
    pub perms: u8,
    pub reset: Option<u32>,
}

impl MPURegType {
    pub fn lookup_offset(offset: usize) -> Option<MPURegType> {
        assert!((offset < 0xd90) || (offset > 0xdec), "offset not in mpu");
        match offset {
            0xd90 => { Some(MPURegType::TYPE) }
            0xd94 => { Some(MPURegType::CTRL) }
            0xd98 => { Some(MPURegType::RNR) }
            0xd9c => { Some(MPURegType::RBAR(0)) }
            0xda0 => { Some(MPURegType::RASR(0)) }
            0xda4 => { Some(MPURegType::RBAR(1)) }
            0xda8 => { Some(MPURegType::RASR(1)) }
            0xdac => { Some(MPURegType::RBAR(2)) }
            0xdb0 => { Some(MPURegType::RASR(2)) }
            0xdb4 => { Some(MPURegType::RBAR(3)) }
            0xdb8 => { Some(MPURegType::RASR(3)) }
            0xdbc..=0xdec => { None /* Reserved. */ }
            _ => { unreachable!() }
        }
    }

    /// returns the byte offset into the system control space of
    /// the mpu register type
    pub fn offset(&self) -> usize {
        self._data().offset
    }

    /// returns access permissions of systick register type
    pub fn permissions(&self) -> u8 {
        self._data().perms
    }

    /// returns mpu register reset value
    pub fn reset_value(&self) -> Option<u32> {
        self._data().reset
    }

    fn _data(&self) -> &'static MPURegData {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From, TryFrom, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum MPUReg {
    TYPE(TYPE),
    CTRL(CTRL),
    RNR(RNR),
    RBAR(u8, RBAR),
    RASR(u8, RASR),
}

#[derive(Debug, Clone, From, TryFrom, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum MPURegRef<'a> {
    TYPE(&'a TYPE),
    CTRL(&'a CTRL),
    RNR(&'a RNR),
    RBAR(u8, &'a RBAR),
    RASR(u8, &'a RASR),
}

#[derive(Debug, From, TryFrom, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum MPURegMut<'a> {
    TYPE(&'a mut TYPE),
    CTRL(&'a mut CTRL),
    RNR(&'a mut RNR),
    RBAR(u8, &'a mut RBAR),
    RASR(u8, &'a mut RASR),
}


/// The MPU Type Register indicates how many regions the MPU supports. 
/// Software can use it to determine if the processor implements an MPU.
/// Word-accessible only. Read-only. Always implemented.
/// 
/// See B3.5.5
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TYPE {
    /// Indicates support for separate instruction and data address maps. 
    /// RAZ. Armv7-M only supports a unified MPU.
    #[bits(1, default = false)]
    pub separate: bool,
    #[bits(7)]
    __: u8,
    /// Number of regions supported by the MPU. 
    /// If this field reads as zero, the processor does not implement an MPU.
    #[bits(8)]
    pub dregion: u8,
    /// Instruction region. RAZ. armv7m only supporst a unified MPU.
    #[bits(8)]
    pub iregion: u8,
    #[bits(8)]
    __: u8,
}

/// Enables the MPU, and when the MPU is enabled, controls whether 
/// the default memory map is enabled as a background region for privileged accesses, 
/// and whether the MPU is enabled for HardFaults, NMIs, and exception handlers 
/// when FAULTMASK is set to 1.
/// Word-accessible only. 
/// If the MPU is not implemented, this register is RAZ/WI.
///
/// See B3.5.6
/// 
/// note: some thumb instructions perform unprivileged memory accesses even when
/// executed by privileged software. See Table B3-12.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CTRL {
    /// Enables the MPU.
    /// - 0: The MPU is disabled.
    /// - 1: The MPU is enabled.
    /// Disabling the MPU, by setting the ENABLE bit to 0, means that 
    /// privileged and unprivileged accesses use the default memory map.
    #[bits(1)]
    pub enable: bool,
    /// Controls whether MPU is enabled for memory accesses by HardFault, NMI, 
    /// and handlers when FAULTMASK is set.
    /// 0: Use the default memory map for memory accesses by these handlers.
    /// 1: Use the MPU for memory accesses by these handlers.
    /// If HFNMIENA is set to 1 when ENABLE is set to 0, behavior is UNPREDICTABLE.
    #[bits(1)]
    pub hfnmiena: bool,
    /// When the ENABLE bit is set to 1, the meaning of this bit is:
    /// 0: Disables the default memory map. Any instruction or data access that does 
    ///    not access a defined region faults.
    /// 1: Enables the default memory map as a background region for privileged access. 
    ///    The background region acts as region number -1. All memory regions configured 
    ///    in the MPU take priority over the default memory map. The system address map on 
    ///    page B3-592 describes the default memory map.
    /// When ENABLE bit is 0, PRIVDEFENA bit is ignored.
    /// If no regions are enabled and PRIVDEFENA and ENABLE are both 1, only
    /// privileged code can execute from the system address map.
    #[bits(1)]
    pub privdefena: bool,
    // Remaining bits are reserved.
    #[bits(29)]
    __: u32,
}

/// Selects the region currently accessed by MPU_RBAR and MPU_RASR.
/// Word-accessible only. Implemented only if the processor implements an MPU.
/// 
/// If an implementation supports N regions, then the regions number from
/// 0 to N - 1, and the effect of writing a value of N or greater
/// to the REGION field is unpredictable
///
/// See B3.5.7
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RNR {
    /// Indicates the memory region accessed by MPU_RBAR and MPU_RASR.
    #[bits(8)]
    pub region: u8,
    // Remaining bits are reserved
    #[bits(24)]
    __: u32,
}

/// Holds the base address of the region identified by MPU_RNR. 
/// On a write, can also be used to update the base address of a specified region, 
/// in the range 0-5, updating MPU_RNR with the new region number.
/// Word-accessible only. Implemented only if the processor implements an MPU.
///
/// See B3.5.8
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RBAR {
    /// On writes, can specify the number of the region to update.
    /// On reads, returns bits[3:0] of MPU_RNR.
    #[bits(4)]
    pub region: u8,
    /// On writes, indicates whether the region to update is specified by MPU_RNR.REGION, 
    /// or by the REGION value specified in this write. When using the REGION value specified 
    /// by this write, MPU_RNR.REGION is updated to this value.
    /// 0: Apply the base address update to the region specified by MPU_RNR.REGION. The REGION field value is ignored.
    /// 1: Update MPU_RNR.REGION to the value obtained by zero extending the REGION value specified in this write, 
    ///    and apply the base address update to this region.
    /// This bit reads as zero.
    #[bits(1)]
    pub valid: bool,
    /// Base address of the region.
    #[bits(27)]
    pub addr: u32,
}

/// Controls the region size, sub-region access, access permissions, memory type, 
/// and other properties of the memory region.
/// Word-accessible only. Implemented only if the processor implements an MPU.
///
/// See B3.5.9
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RASR {
    /// Enables this region:
    /// 0: When the MPU is enabled, this region is disabled.
    /// 1: When the MPU is enabled, this region is enabled.
    /// Enabling a region has no effect unless the MPU_CTRL.ENABLE bit is set to 1, to enable the MPU.
    #[bits(1)]
    pub enable: bool,
    /// Indicates the region size. The region size, in bytes, is 2^(SIZE+1). 
    /// SIZE field values less than 4 are reserved because the smallest supported 
    /// region size is 32 bytes.
    #[bits(5)]
    pub size: u32,
    // Reserved bits
    #[bits(2)]
    __: u8,
    /// Subregion Disable. For regions of 256 bytes or larger, each bit of this 
    /// field controls whether one of the eight equal subregions is enabled.
    /// 0: Subregion enabled
    /// 1: Subregion disabled
    #[bits(8)]
    pub srd: u8,
    /// Bufferable.
    #[bits(1)]
    pub b: bool,
    /// Cacheable.
    #[bits(1)]
    pub c: bool,
    /// Shareable.
    #[bits(1)]
    pub s: bool,
    /// Type Extension, three bits.
    #[bits(3)]
    pub tex: u8,
    #[bits(2)]
    __: u8,
    /// Access Permissions, three bits.
    #[bits(3)]
    pub ap: u8,
    #[bits(1)]
    __: bool,
    /// Execute Never.
    #[bits(1)]
    pub xn: bool,
    #[bits(3)]
    __: u8,
}



impl MPURegType {
    pub(super) unsafe fn to_reg_ref<'a>(&self, int_ref: &'a u32) -> MPURegRef {
        todo!()
    }

    pub(super) unsafe fn to_reg_mut<'a>(&self, int_ref: &'a mut u32) -> MPURegMut {
        todo!()
    }
}