//! test.rs
//! 
//! dft module tests
use crate::backend::Backend;
use crate::utils::*;
use crate::test::programs;

#[test]
fn test_smash_stack() -> Result<(), anyhow::Error> {
    use std::sync::Arc;
    use fugue_core::prelude::*;
    use fugue_core::ir::Location;
    use fugue_ir::disassembly::IRBuilderArena;
    use crate::types::Permission;
    use crate::programdb::{self, ProgramDB, Region, Platform};
    use crate::backend::armv7m;
    use crate::dft::{
        self,
        Evaluator,
        tag::{self, Tag},
        policy::jump::*,
    };

    let global_sub = compact_dbg_file_logger("test_smash_stack.log");
    set_global_default(global_sub)
        .expect("failed to set tracing default logger");

    let program = programs::STACK_SMASH_TEST;

    info!("initializing programdb...");
    let platform = Platform {
        name: "dummy".into(),
        cpu_name: "CM3".into(),
        cpu_revision: "".into(),
        cpu_endian: Endian::Little,
        mpu_present: false,
        fpu_present: false,
        nvic_prio_bits: 8,
        vendor_systick_config: false,
        mem: vec![Region {
            name: "memory".into(),
            address: 0x0u64.into(),
            size: 0x1000,
            perms: Permission::R | Permission::W,
            description: "".into(),
        }],
        mmio: vec![],
    };
    let builder = LanguageBuilder::new("data/processors")?;
    let irb = IRBuilderArena::with_capacity(0x1000);
    let mut pdb = ProgramDB::new_with(&builder, platform, &irb);

    info!("building backend...");
    let backend = armv7m::Backend::new_with(&builder, None)?;
    let lang = Arc::new(backend.lang().clone());
    let policy = TaintedJumpPolicy::new_with(lang);

    info!("building dft context...");
    let mut context = dft::Context::new_with(Box::new(backend));

    info!("mapping memory...");
    context.map_mem(0x0u64, 0x1000)?;

    info!("loading program...");
    context.store_bytes(0x0u64, program, &Tag::from(tag::UNACCESSED))?;

    info!("initializing taint...");
    let tainted_data_address = 0x40u64;
    context.write_tags(tainted_data_address, 4, tag::TAINTED_VAL)?;
    let tags = context.view_tags(tainted_data_address, 4)?;
    tags.iter().for_each(|tag| {
        assert!(tag.is_tainted(), "tag should be tainted");
    });

    info!("initializing program...");
    context.write_sp(0x1000u64, &Tag::from(tag::ACCESSED))?;
    context.write_pc(0u64, &Tag::from(tag::ACCESSED))?;

    info!("initializing dummy plugins...");
    let eval_plugin = Box::new(dft::plugin::DummyEvalPlugin::default());
    let pdb_plugin = Box::new(programdb::plugin::DummyAnalysisPlugin::default());
    pdb.add_plugin(pdb_plugin);

    info!("executing program...");
    let mut evaluator = Evaluator::new_with_policy(&policy);
    evaluator.add_plugin(eval_plugin);
    (evaluator.pc, evaluator.pc_tag) = context.read_pc()
        .map(|(pc, tag)| (Location::from(pc), tag))?;

    let mut cycles = 0;
    while cycles < 500 {
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

    // expect a failure to occur
    Err(anyhow::Error::msg("expected a policy violation"))
}