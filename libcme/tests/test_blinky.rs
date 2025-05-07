use std::fs;
use anyhow;

use test_log::test;

#[test]
fn test_blinky() -> Result<(), anyhow::Error> {
    use std::sync::Arc;
    use libcme::prelude::*;
    use tracing::subscriber::set_global_default;
    // use libcme::peripheral::{ self, * };

    let global_sub = compact_dbg_file_logger("test_blinky.log");
    set_global_default(global_sub)
        .expect("failed to set tracing default logger");

    info!("loading program binary...");
    let program = &fs::read("tests/samples/nrf52840dk/blinky/blinky.bin")?;

    info!("creating language builder...");
    let builder = LanguageBuilder::new("data/processors")?;
    let irb = IRBuilderArena::with_capacity(0x1000);
    
    info!("building programdb...");
    let platform = Platform::from_path("tests/samples/nrf52840dk/nrf52840.yml")?;
    let mut pdb = ProgramDB::new_with(&builder, platform, &irb);

    info!("building backend...");
    let backend = pdb.backend(&builder)?;
    let lang = Arc::new(backend.lang().clone());
    let policy = policy::TaintedJumpPolicy::new_with(lang);

    info!("building dft context...");
    let mut context = dft::Context::from_backend(backend)?;

    // // note that memory is specified in yml config
    // info!("mapping memory...");
    // // map flash
    // context.map_mem(0x0u64, 0x100000usize)?;
    // // map data ram
    // context.map_mem(0x20000000u64, 0x40000usize)?;

    info!("loading program...");
    context.store_bytes(0x0u64, program, &dft::Tag::from(tag::UNACCESSED))?;

    info!("initializing program...");
    // read sp and load entrypoint
    let stack_size = bytes_as_u32_le(&program[..4]);
    let entry = bytes_as_u32_le(&program[4..8]);

    context.write_sp(stack_size, &dft::Tag::from(tag::ACCESSED))?;
    context.write_pc(entry, &dft::Tag::from(tag::ACCESSED))?;

    info!("initializing dummy plugins...");
    let eval_plugin = Box::new(dft::plugin::DummyEvalPlugin::default());
    let pdb_plugin = Box::new(programdb::plugin::DummyAnalysisPlugin::default());
    pdb.add_plugin(pdb_plugin);

    info!("executing program...");
    let mut evaluator = dft::Evaluator::new_with_policy(&policy);
    evaluator.add_plugin(eval_plugin);
    (evaluator.pc, evaluator.pc_tag) = context.read_pc()
        .map(|(pc, tag)| (Location::from(pc), tag))?;

    let mut cycles = 0;
    while cycles < 10000 {
        let result = evaluator.step(&mut context, &mut pdb);
        match result {
            Err(dft::eval::Error::Policy(err)) => {
                error!("policy violation: {err:?}");
                return Ok(())
            }
            Err(err) => {
                error!("other evaluator error: {err:?}");
                return Err(anyhow::Error::from(err))
            }
            _ => {
                cycles += 1;
                if evaluator.pc.address() == Address::from(0x06u64) {
                    break;
                }
            }
        }
    }

    Ok(())
}