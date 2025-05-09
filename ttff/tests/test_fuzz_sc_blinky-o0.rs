use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow;
use libcme::prelude::*;
use libcme::peripheral::channel::*;
use ttff::prelude::*;

use libafl_bolts::{
    rands::StdRand,
    tuples::tuple_list,
    nonzero,
};
use libafl::{
    self,
    corpus::OnDiskCorpus,
    events::{
        // Launcher,
        // EventConfig,
        SimpleEventManager,
        // SendExiting,
        // ShutdownSignalData,
    },
    executors::{ExitKind, WithObservers},
    feedback_or, feedback_or_fast, feedback_and,
    feedbacks::{CrashFeedback, MaxMapFeedback, TimeFeedback, TimeoutFeedback},
    fuzzer::{Fuzzer, StdFuzzer},
    generators::{RandBytesGenerator, RandPrintablesGenerator},
    inputs::{BytesInput, HasTargetBytes},
    monitors::{MultiMonitor, SimpleMonitor},
    mutators::{
        havoc_mutations::havoc_mutations,
        scheduled::StdScheduledMutator,
    },
    observers::{
        CanTrack,
        HitcountsMapObserver,
        StdMapObserver,
        TimeObserver,
    },
    schedulers::{
        powersched::PowerSchedule,
        StdWeightedScheduler,
        IndexesLenTimeMinimizerScheduler,
        QueueScheduler,
    },
    stages::{
        calibrate::CalibrationStage,
        power::StdPowerMutationalStage,
        AflStatsStage,
        GeneralizationStage,
        StdMutationalStage,
        TracingStage,
    },
    state::{StdState, HasCorpus},
    Evaluator,
};

// use lazy_static::lazy_static;

const COVMAP_SIZE: usize = 0x2000;
static mut COVMAP: [u8; COVMAP_SIZE] = [0u8; COVMAP_SIZE];


type MemCallback = dyn FnMut(
    &mut dft::Context,
    &Address,
    usize,
) -> Result<(), dft::plugin::Error>;

pub struct MemInterceptPlugin<'a> {
    pub callback: &'a mut MemCallback,
}

impl<'a> std::fmt::Debug for MemInterceptPlugin<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MemInterceptPlugin")
    }
}

impl<'a> dft::EvalPlugin for MemInterceptPlugin<'a> {
    #[instrument(skip_all)]
    fn pre_mem_access_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        mem_address: &Address,
        mem_size: usize,
        _access_type: Permission,
        context: &mut dft::Context<'backend>,
    ) -> Result<(), dft::plugin::Error> {
        if !context.backend().mmap().has_mapped(mem_address) {
            error!("encountered unmapped access @ {:#x}", mem_address.offset());
            (self.callback)(context, mem_address, mem_size)?;
        }
        Ok(())
    }
}


#[test]
pub fn main() -> Result<(), anyhow::Error> {
    let (global_sub, _guard) = compact_file_logger(
        "tests/test_fuzz_jump_sc_blinky-o0.log",
        Level::INFO);
    set_global_default(global_sub)
        .expect("failed to set tracing default logger");

    let irb = IRBuilderArena::with_capacity(0x10000);

    let covmap = CovMap::new(
        #[allow(static_mut_refs)]
        unsafe { &mut COVMAP as *mut [u8] },
        COVMAP_SIZE,
    );

    info!("reading program binary...");
    let bytes = fs::read("data/nrf52/blinky-o0.elf")?;
    let elf_bytes = ElfBytes::minimal_parse(bytes.as_slice())?;
    let program = Program::new_from_elf(irb.inner(), elf_bytes)?;

    info!("creating language builder...");
    let builder = LanguageBuilder::new("data/processors")?;

    info!("building programdb...");
    let platform = Platform::from_path("data/nrf52/nrf52.yml")?;
    let mut pdb = ProgramDB::new_with(&builder, program, platform, &irb);

    let hc_plugin = HcPlugin::new(covmap);
    pdb.add_plugin(Box::new(hc_plugin));

    info!("building backend...");
    let backend = pdb.backend(&builder)?;

    info!("building context...");
    let mut context = dft::Context::from_backend(backend)?;

    info!("mapping channel peripherals");
    struct Peri { name: String, base: Address, size: usize, tag: dft::Tag }
    let mut peripherals: Vec<Peri> = vec![];
    for &MmioRegion {
        ref name,
        base,
        blocksize,
        perms: _, // still need to implement mpu
        description: _,
    } in pdb.platform().mmio().iter() {
        debug!("processing peripheral: {name}");
        let mmio_tag: dft::Tag = match name.as_str() {
            "p0" | "gpiote" => { tag::TAINTED_VAL.into() }
            _ => { tag::ACCESSED.into() }
        };
        // construct the combined peripherals to account for overlapping peripheral regions
        // overlapping peripheral regions will be combined and their taint tags or'd together
        // this is a really dumb way to deal with overlaps.
        // it would be much better to refactor peripherals to handle it better...
        let range = base..(base + blocksize as u64);
        let mut overlaps = false;
        for peri in peripherals.iter_mut() {
            let peri_range = peri.base..(peri.base + peri.size as u64);
            if range.start < peri_range.end && peri_range.start < range.end {
                // merge overlapping peripheral
                peri.base = std::cmp::min(base, peri.base);
                peri.size = (
                    std::cmp::max(range.end.offset(), peri_range.end.offset())
                    .saturating_sub(peri.base.offset())) as usize;
                peri.tag |= mmio_tag;
                overlaps = true;
                break;
            }
        }
        if !overlaps {
            peripherals.push(Peri {
                name: name.clone(),
                base: base.clone(),
                size: blocksize,
                tag: mmio_tag
            });
        }
    }

    // map peripheral ranges with channel peripherals
    let GeneratedChannelPeripheral {
        access_log,
        read_src,
        write_dst,
        peripheral,
    } = ChannelPeripheral::new(Address::default(), 0x1000);
    for peri in peripherals {
        info!("mapping peripheral {} @ [{:#x}, {:#x})",
            peri.name, peri.base.offset(), peri.size);
        let new_peripheral = peripheral.clone_with(peri.base, peri.size);
        context.map_mmio(new_peripheral.into(), Some(peri.tag))?;
    }
    let callback = &mut move |context: &mut dft::Context, address: &Address, size: usize| {
        let base = address.offset() & 0xFFFFF000;
        let size = ((size / 0x1000) + 1) * 0x1000;
        if (0x20000000 <= base && base < 0x40000000)
            || (0x60000000 <= base && base < 0xA0000000) {
            // ram memory should never be dynamically mapped with a peripheral
            return Ok(())
        }
        info!("dynamically mapping new channel peripheral @ [{base:#x}; {size:#x}]");
        let new_peripheral = peripheral.clone_with(base, size);
        context.map_mmio(new_peripheral.into(), Some(tag::ACCESSED.into()))
            .map_err(|err| dft::plugin::Error(err.into()))
    };
    let unmapped_plugin = MemInterceptPlugin { callback };

    info!("loading program binary...");
    // this can probably be absorbed into the pdb backend builder
    for section in pdb.program().loadable_sections() {
        context.store_bytes(
            section.address(),
            section.data(),
            &dft::Tag::from(tag::UNACCESSED),
        )?;
    }

    info!("initializing context...");
    // this should be absorbed into the context reset
    let mut stack_bytes = [0u8; 4];
    context.load_bytes(0u64, &mut stack_bytes)?;
    let stack_top = u32::from_le_bytes(stack_bytes);
    context.write_sp(stack_top, &dft::Tag::from(tag::ACCESSED))?;

    let mut entry_bytes = [0u8; 4];
    context.load_bytes(4u64, &mut entry_bytes)?;
    let entry = u32::from_le_bytes(entry_bytes);
    context.write_pc(entry, &dft::Tag::from(tag::ACCESSED))?;

    info!("building taint policy...");
    let lang = Arc::new(pdb.lang().clone());
    let policy = ttff::policy::TaintedJumpPolicy::new_with(lang);

    info!("building evaluator...");
    let mut evaluator = dft::Evaluator::new_with_policy(&policy);
    evaluator.add_plugin(Box::new(unmapped_plugin));
    (evaluator.pc, evaluator.pc_tag) = context.read_pc()
        .map(|(pc, tag)| (Location::from(pc), tag))?;

    info!("building dft executor...");
    let halt_fn = None;
    let limit = Some(50000 as usize);
    let exc_limit = Some(200);

    let dft_executor = sc::DftExecutor::new_with(
        evaluator,
        context,
        pdb,
        limit,
        exc_limit,
        halt_fn,
        access_log.clone(),
        read_src.clone(),
        write_dst.clone(),
    );

    info!("building libafl observers, feedbacks, and objective...");
    let edges_observer = unsafe {
        #[allow(static_mut_refs)]
        HitcountsMapObserver::new(StdMapObserver::new("edges", &mut COVMAP))
            .track_indices()
    };
    let time_observer = TimeObserver::new("time");
    let map_feedback = MaxMapFeedback::new(&edges_observer);
    let mut feedback = feedback_or!(
        // maximize coverage
        map_feedback,
        TimeFeedback::new(&time_observer),
    );
    let mut objective = feedback_or_fast!(
        CrashFeedback::new(),
        TimeoutFeedback::new(),
    );

    info!("building stages...");
    let mutator = StdScheduledMutator::new(havoc_mutations());
    let mut stages = tuple_list!(
        StdMutationalStage::new(mutator),
        AflStatsStage::builder()
            .map_observer(&edges_observer)
            .build()
            .unwrap(),
    );

    info!("building libafl state...");
    let queue_corpus_path = PathBuf::from("tests/fuzz_sc_blinky-o0/queue");
    let crash_corpus_path = PathBuf::from("tests/fuzz_sc_blinky-o0/crashes");
    let mut state = StdState::new(
        StdRand::with_seed(42),
        // OnDiskCorpus::new(queue_corpus_path)?,
        OnDiskCorpus::<BytesInput>::new(queue_corpus_path)?,
        OnDiskCorpus::new(crash_corpus_path)?,
        &mut feedback,
        &mut objective,
    )?;

    info!("building monitor...");
    let monitor = MultiMonitor::new(|s| info!("{s}"));
    let mut manager = SimpleEventManager::new(monitor);

    info!("building scheduler, and fuzzer...");
    let scheduler = QueueScheduler::new();
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    let mut executor = WithObservers::new(
        dft_executor,
        tuple_list!(edges_observer, time_observer),
    );

    let mut generator = RandBytesGenerator::new(nonzero!(0x10000));
    state
        .generate_initial_inputs(&mut fuzzer, &mut executor, &mut generator, &mut manager, 8)
        .expect("failed to generate initial corpus");

    match fuzzer.fuzz_loop(&mut stages, &mut executor, &mut state, &mut manager) {
        Err(libafl::Error::ShuttingDown) => { error!("fuzzer stopped by user."); Ok(()) }
        Err(err) => {
            error!("failed to run launcher: {err:?}");
            Err(err.into())
        }
        _ => { Ok(()) }
    }
}