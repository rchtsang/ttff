use std::fs;
use anyhow;

use test_log::test;

#[test]
fn test_blinky() -> Result<(), anyhow::Error> {
    use std::sync::Arc;
    use elf::ElfBytes;
    use libcme::prelude::*;
    use tracing::subscriber::set_global_default;
    // use libcme::peripheral::{ self, * };

    let global_sub = compact_dbg_file_logger("test_blinky.log");
    set_global_default(global_sub)
        .expect("failed to set tracing default logger");

    let irb = IRBuilderArena::with_capacity(0x1000);

    info!("loading program binary...");
    let bytes = fs::read("tests/samples/nrf52840dk/blinky/blinky.elf")?;
    let elf_bytes = ElfBytes::minimal_parse(bytes.as_slice())?;
    let program = Program::new_from_elf(
        irb.inner(),
        elf_bytes,
    )?;
    
    info!("creating language builder...");
    let builder = LanguageBuilder::new("data/processors")?;
    
    info!("building programdb...");
    let platform = Platform::from_path("tests/samples/nrf52840dk/nrf52840.yml")?;
    let mut pdb = ProgramDB::new_with(&builder, program, platform, &irb);

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
    for section in pdb.program().loadable_sections() {
        context.store_bytes(
            section.address(),
            section.data(),
            &dft::Tag::from(tag::UNACCESSED),
        )?;
    }

    info!("initializing program...");
    // read sp and load entrypoint
    let mut stack_bytes = [0u8; 4];
    context.load_bytes(0u64, &mut stack_bytes)?;
    let stack_size = u32::from_le_bytes(stack_bytes);
    let mut entry_bytes = [0u8; 4];
    context.load_bytes(4u64, &mut entry_bytes)?;
    let entry = u32::from_le_bytes(entry_bytes);

    context.write_sp(stack_size, &dft::Tag::from(tag::ACCESSED))?;
    context.write_pc(entry, &dft::Tag::from(tag::ACCESSED))?;

    info!("initializing dummy plugins...");
    let eval_plugin = Box::new(dft::plugin::DummyEvalPlugin::default());
    let pdb_plugin = Box::new(programdb::plugin::DummyAnalysisPlugin::default());
    pdb.add_plugin(pdb_plugin);

    info!("executing program...");
    let mut evaluator = dft::Evaluator::new_with_policy(Box::new(policy));
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