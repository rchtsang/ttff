use std::fs;
use std::path::PathBuf;
use std::ops::Range;
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


#[test]
pub fn main() -> Result<(), anyhow::Error> {
    let global_sub = compact_dbg_file_logger("test_fuzz_jump_sc_blinky-o0");
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
    let mut peripherals: Vec<((String, Address, usize), Vec<(Range<Address>, dft::Tag)>)> = vec![];
    for &MmioRegion {
        ref name,
        base,
        blocksize,
        ref ranges,
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
        let mut overlaps = false;
        for (other_peripheral, other_ranges) in peripherals.iter_mut() {
            assert_ne!(other_peripheral.0.as_str(), name.as_str(), "same peripheral names not allowed");
            let mut to_add = vec![];
            for range in ranges.iter() {
                let mut added = false;
                for (other_range, other_tag) in other_ranges.iter_mut() {
                    if range.start < other_range.end && other_range.start < range.end {
                        other_range.start = std::cmp::min(other_range.start, range.start);
                        other_range.end = std::cmp::max(other_range.end, range.end);
                        *other_tag |= mmio_tag;
                        overlaps = true;
                        added = true;
                    }
                }
                if added {
                    // other ranges may now be overlapping with themselves.
                    // if this is the case, merge any overlapping again.
                    other_ranges.sort_by(|(r1, _t1), (r2, _t2)| {
                        (r1.start.offset(), r1.end.offset()).cmp(&(r2.start.offset(), r2.end.offset()))
                    });
                    let mut i = 0;
                    while i < other_ranges.len() {
                        let mut j = i + 1;
                        while j < other_ranges.len() {
                            if other_ranges[i].0.start < other_ranges[j].0.end && other_ranges[j].0.start < other_ranges[i].0.end {
                                other_ranges[i].0.start = std::cmp::min(other_ranges[i].0.start, other_ranges[j].0.start);
                                other_ranges[i].0.end = std::cmp::max(other_ranges[i].0.end, other_ranges[j].0.end);
                                other_ranges.remove(j);
                            } else {
                                j += 1;
                            }
                        }
                        i += 1;
                    }
                } else {
                    // if not added, save this range to be added later
                    to_add.push(range.clone());
                }
            }
            if overlaps {
                // if there was an overlap, combined the peripherals
                // and break the loop
                debug!("merging with existing peripheral: {}...",
                    other_peripheral.0);
                let to_add = to_add.into_iter()
                    .map(move |r| (r, mmio_tag.clone()));
                other_ranges.extend(to_add);
                other_ranges.sort_by(|(r1, _t1), (r2, _t2)| {
                    (r1.start.offset(), r1.end.offset()).cmp(&(r2.start.offset(), r2.end.offset()))
                });
                break;
            }
        }
        if !overlaps {
            // if there were no overlaps, create a new associated peripheral
            debug!("no overlaps, creating new peripheral...");
            let tagged_ranges: Vec<(Range<Address>, dft::Tag)> = ranges.iter().cloned()
                .map(move |r| (r, mmio_tag.clone()))
                .collect();
            peripherals.push(((name.clone(), base.clone(), blocksize), tagged_ranges));
        }
    }

    // map peripheral ranges with channel peripherals
    let GeneratedChannelPeripheral {
        access_log,
        read_src,
        write_dst,
        peripheral,
    } = ChannelPeripheral::new(Address::default(), 0x1000, vec![].iter());
    for (peri, tagged_ranges) in peripherals {
        info!("mapping peripheral {}...", peri.0);
        for (r, _t) in tagged_ranges.iter() {
            debug!("range: [{:#x}, {:#x})", r.start.offset(), r.end.offset());
        }
        let tag = tagged_ranges.iter().fold(dft::Tag::from(tag::ACCESSED), |v, (_r, t)| {
            v | *t
        });
        let ranges = tagged_ranges.iter().map(|(r, _t)| r);
        let new_peripheral = peripheral.clone_with(peri.1, peri.2, ranges);
        context.map_mmio(new_peripheral.into(), Some(tag))?;
    }

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
    (evaluator.pc, evaluator.pc_tag) = context.read_pc()
        .map(|(pc, tag)| (Location::from(pc), tag))?;

    info!("building dft executor...");
    let halt_fn = None;
    let limit = Some(100000 as usize);

    let dft_executor = sc::DftExecutor::new_with(
        evaluator,
        context,
        pdb,
        limit,
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
    let monitor = MultiMonitor::new(|s| println!("{s}"));
    let mut manager = SimpleEventManager::new(monitor);

    info!("building scheduler, and fuzzer...");
    let scheduler = QueueScheduler::new();
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    let mut executor = WithObservers::new(dft_executor, tuple_list!(edges_observer));

    let mut generator = RandBytesGenerator::new(nonzero!(0x1000));
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