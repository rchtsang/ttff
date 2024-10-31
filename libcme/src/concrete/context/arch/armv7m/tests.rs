//! tests.rs

use super::*;
use crate::concrete::tests;
use crate::peripheral::{Peripheral, dummy::DummyState};

#[test]
fn test_read_write() -> Result<(), context::Error> {
    let builder = LanguageBuilder::new("data/processors")?;
    let irb = IRBuilderArena::with_capacity(0x1000);
    let mut context = Context::new_with(&builder, &irb, None)?;
    context.map_mem(0x0u64, 0x1000usize)?;

    // test map dummy peripheral
    let dummy_base = Address::from(0x2000u64);
    let dummy = Peripheral::new_with(dummy_base..(dummy_base + 0x400u64), Box::new(DummyState::default()));
    context.map_mmio(dummy)?;

    // test read/write mem bytes
    context._map_write_bytes(Address::from(0x0u64), tests::TEST_PROG_SQUARE)?;
    let mut bytes = [0u8; 4];
    context._map_read_bytes(Address::from(0x0u64), &mut bytes)?;
    assert_eq!(bytes, [0x00, 0xf0, 0x01, 0xf8], "read incorrect byte sequence: {bytes:#x?}");

    // test read/write mem values
    let addr = Address::from(0x100u64);
    let val = BitVec::from_u64(0xdeadbeefu64, 32);
    context._map_write_val(addr, &val)?;
    let bv = context._map_read_val(addr, val.bytes())?;
    assert_eq!(bv, val, "read incorrect bitvec: {bv:#x?}");

    // test read/write varnodes
    let t = context.translator();
    let r0_vnd = t.register_by_name("r0")
        .expect("r0 not a register???");
    context._write_vnd(&r0_vnd, &val)?;
    let bv = context._read_vnd(&r0_vnd)?;
    assert_eq!(bv, val, "read incorrect bitvec: {bv:#x?}");

    // test fetch
    let _insn = context._fetch(0x0u64)?;

    Ok(())
}

#[test]
fn test_requests() -> Result<(), context::Error> {
    let builder = LanguageBuilder::new("data/processors")?;
    let irb = IRBuilderArena::with_capacity(0x1000);
    let mut context = Context::new_with(&builder, &irb, None)?;
    context.map_mem(0x0u64, 0x1000usize)?;

    // request load/store program bytes
    let addr = Address::from(0x0u64);
    context.store_bytes(addr, tests::TEST_PROG_SQUARE)?;
    let mut bytes = [0u8; 4];
    context.load_bytes(addr, &mut bytes)?;
    assert_eq!(bytes, [0x00, 0xf0, 0x01, 0xf8], "read incorrect byte sequence: {bytes:#x?}");

    // request load/store val
    let addr = Address::from(0x100u64);
    let val = BitVec::from_u64(0xdeadbeefu64, 32);
    context.store(addr, &val)?;
    let bv = context.load(addr, val.bytes())?;
    assert_eq!(bv, val, "read incorrect bitvec: {bv:#x?}");

    // request read/write varnode
    let t = context.translator();
    let r0_vnd = t.register_by_name("r0")
        .expect("r0 not a register???");
    context.write(&r0_vnd, &val)?;
    let bv = context.read(&r0_vnd)?;
    assert_eq!(bv, val, "read incorrect bitvec: {bv:#x?}");

    // test fetch
    let _insn = context.fetch(0x0u64)?;

    Ok(())
}

#[test]
fn test_systick_registers() -> Result<(), context::Error> {
    let builder = LanguageBuilder::new("data/processors")?;
    let irb = IRBuilderArena::with_capacity(0x1000);
    let mut context = Context::new_with(&builder, &irb, None)?;
    context.map_mem(0x0u64, 0x1000usize)?;

    let address = SysTickRegType::CSR.address();
    let bytes = u32::to_le_bytes(0b11);
    context.store_bytes(address, &bytes)?;

    assert_eq!(context.events.len(), 1);
    assert_eq!(context.events[0], Event::ExceptionEnabled(ExceptionType::SysTick, true));

    let mut dst = [0u8; 4];
    context.load_bytes(address, &mut dst)?;

    assert_eq!(u32::from_le_bytes(dst), 0b11);

    Ok(())
}