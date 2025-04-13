//! tests.rs

use super::*;
use crate::backend;
// use crate::peripheral::{Peripheral, dummy::DummyState};
use crate::test;
// use crate::utils::*;

#[test]
fn test_requests() -> Result<(), backend::Error> {
    let global_sub = compact_dbg_logger();
    set_global_default(global_sub)
        .expect("failed to set tracing default logger");

    info!("in directory: {}",
        std::env::current_dir()
            .expect("failed to get current directory")
            .display());

    info!("creating language builder...");
    let metadata = (std::fs::metadata("data/processors"))
        .expect("failed to find processors directory");
    assert!(metadata.is_dir(), "path is not a directory");

    let builder = LanguageBuilder::new("data/processors")?;
    let irb = IRBuilderArena::with_capacity(0x1000);

    info!("building backend...");
    let mut backend = Backend::new_with(&builder, &irb, None)?;

    info!("mapping memory...");
    backend.map_mem(&Address::from(0x0u64), 0x1000usize)?;

    info!("testing load/store bytes...");
    let addr = Address::from(0x0u64);
    backend.store_bytes(&addr, test::programs::TEST_PROG_SQUARE)?;
    let mut bytes = [0u8; 4];
    backend.load_bytes(&addr, &mut bytes)?;
    assert_eq!(bytes, [0x00, 0xf0, 0x01, 0xf8], "read incorrect byte sequence: {bytes:#x?}");

    info!("testing load/store values...");
    let addr = Address::from(0x100u64);
    let val = BitVec::from_u64(0xdeadbeefu64, 32);
    backend.store(&addr, &val)?;
    let bv = backend.load(&addr, val.bytes())?;
    assert_eq!(bv, val, "read incorrect bitvec: {bv:#x?}");

    info!("testing read/write varnodes...");
    let t = backend.translator();
    let r0_vnd = t.register_by_name("r0")
        .expect("r0 not a register???");
    backend.write(&r0_vnd, &val)?;
    let bv = backend.read(&r0_vnd)?;
    assert_eq!(bv, val, "read incorrect bitvec: {bv:#x?}");

    info!("testing fetch...");
    let _insn = backend.fetch(&Address::from(0x0u64))
        .expect("failed to fetch instruction");

    info!("done.");
    Ok(())
}

#[test]
fn test_systick_registers() -> Result<(), backend::Error> {
    let global_sub = compact_dbg_logger();
    set_global_default(global_sub)
        .expect("failed to set tracing default logger");

    info!("creating language builder...");
    let builder = LanguageBuilder::new("data/processors")?;
    let irb = IRBuilderArena::with_capacity(0x1000);

    info!("building backend...");
    let mut backend = Backend::new_with(&builder, &irb, None)?;

    info!("mapping memory...");
    backend.map_mem(&Address::from(0x0u64), 0x1000usize)?;

    info!("writing to CSR...");
    let address = SysTickRegType::CSR.address();
    let bytes = u32::to_le_bytes(0b11);
    backend.store_bytes(&address, &bytes)?;

    info!("checking for generated systick event...");
    assert_eq!(backend.events.len(), 1);
    assert_eq!(backend.events[0], Event::ExceptionEnabled(ExceptionType::SysTick, true));

    info!("reading from CSR...");
    let mut dst = [0u8; 4];
    backend.load_bytes(&address, &mut dst)?;

    assert_eq!(u32::from_le_bytes(dst), 0b11);

    info!("done.");
    Ok(())
}