//! uart-jump demo
//! 
use std::fs;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow;
use crossbeam::channel::unbounded;
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
    feedback_or, feedback_or_fast,
    feedbacks::{CrashFeedback, MaxMapFeedback, TimeFeedback, TimeoutFeedback},
    fuzzer::{Fuzzer, StdFuzzer},
    generators::RandBytesGenerator,
    inputs::BytesInput,
    monitors::MultiMonitor,
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
    schedulers::QueueScheduler,
    stages::{
        AflStatsStage,
        StdMutationalStage,
    },
    state::StdState,
};

use libcme::prelude::*;
use ttff::prelude::*;

pub mod ficr;
pub mod uicr;
pub mod uart;
pub mod gpio;

const COVMAP_SIZE: usize = 0x2000;
static mut COVMAP: [u8; COVMAP_SIZE] = [0u8; COVMAP_SIZE];

use crossbeam::channel::{
    Sender,
    Receiver,
};
use ttff::policy::address::{FrameUpdate, FrameStart};

#[derive(Debug)]
pub struct CallStackPlugin {
    pub callstack: VecDeque<Address>,
    pub call_channel: (Sender<FrameUpdate>, Receiver<FrameUpdate>),
}

impl Default for CallStackPlugin {
    fn default() -> Self {
        let callstack = VecDeque::default();
        let call_channel = unbounded();
        Self { callstack, call_channel }
    }
}

impl EvalPlugin for CallStackPlugin {

    #[instrument(skip_all)]
    fn post_insn_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        _insn: &Insn<'irb>,
        flow: &Flow,
        context: &mut dft::Context<'backend>,
        _pdb: &mut ProgramDB<'irb>,
    ) -> Result<(), dft::plugin::Error> {
        match flow.flowtype {
            FlowType::Call
            | FlowType::ICall => {
                let target = flow.target.unwrap().address();
                debug!("calling {:#x} from {:#x}",
                    target.offset(),
                    self.callstack.back().unwrap_or(&0u64.into()).offset());
                self.callstack.push_back(target);
                let frame_start = FrameStart {
                    pc: target,
                    sp: context.backend().read_sp().unwrap(),
                };
                let _ = self.call_channel.0.try_send(FrameUpdate::Call(frame_start));
                Ok(())
            }
            FlowType::Return => {
                self.callstack.pop_back().unwrap();
                let _ = self.call_channel.0.try_send(FrameUpdate::Return);
                Ok(())
            }
            _ => { Ok(()) }
        }
    }
}

pub fn main() -> Result<(), anyhow::Error> {
    let (global_sub, _guard) = compact_file_logger(
        "examples/uart-address/uart-address.log",
        Level::TRACE,
    );
    set_global_default(global_sub)?;

    // configure test fuzz run limits
    let limit = Some(1000000 as usize);
    let exc_limit = Some(5);

    let irb = IRBuilderArena::with_capacity(0x10000);

    let covmap = CovMap::new(
        #[allow(static_mut_refs)]
        unsafe { &mut COVMAP as *mut [u8] },
        COVMAP_SIZE,
    );

    info!("reading program binary...");
    let path = "examples/samples/uart-address/uart-address.elf";
    let bytes = fs::read(path)?;
    let elf_bytes = ElfBytes::minimal_parse(bytes.as_slice())?;
    let program = Program::new_from_elf(irb.inner(), elf_bytes)?;

    info!("creating language builder...");
    let builder = LanguageBuilder::new("data/processors")?;
    
    info!("building programdb...");
    let platform = Platform::from_path("data/nrf52/nrf52.yml")?;
    let mut pdb = ProgramDB::new_with(&builder, program, platform, &irb);

    let hc_plugin = HcPlugin::new(covmap);
    pdb.add_plugin(Box::new(hc_plugin));

    info!("building context...");
    let backend = pdb.backend(&builder)?;
    let mut context = dft::Context::from_backend(backend)?;

    info!("mapping peripherals...");
    let ficr_peripheral = ficr::FICRState::new_with(ficr::FICR_BASE);
    let uicr_peripheral = uicr::UICRState::new_with(uicr::UICR_BASE);
    let gpio_peripheral = gpio::GPIOState::new_with(gpio::P0_BASE);
    context.map_mmio(Peripheral::new_with(Box::new(ficr_peripheral)), None)?;
    context.map_mmio(Peripheral::new_with(Box::new(uicr_peripheral)), None)?;
    context.map_mmio(Peripheral::new_with(Box::new(gpio_peripheral)), None)?;
    
    let access_log = unbounded();
    let tx_channel = unbounded();
    let rx_channel = unbounded();
    let uart_peripheral = uart::UARTState::new_with(
        access_log.clone(), rx_channel.clone(), tx_channel.clone());
    context.map_mmio(
        Peripheral::new_with(Box::new(uart_peripheral)),
        Some(dft::Tag::from(tag::TAINTED_VAL)),
    )?;

    for mapped_range in context.backend().mmap().mapped() {
        match mapped_range {
            MappedRange::Mem(range) => {
                info!("mapped mem: [{:#x}, {:#x}]",
                    range.start.offset(), range.end.offset());
            }
            MappedRange::Mmio(range) => {
                info!("mapped mmio: [{:#x}, {:#x}]",
                    range.start.offset(), range.end.offset());
            }
        }
    }

    info!("loading program binary...");
    for segment in pdb.program().loadable_segments() {
        context.store_bytes(
            segment.p_paddr(),
            segment.data(),
            &dft::Tag::from(tag::UNACCESSED),
        )?;
    }

    info!("initializting context...");
    let mut stack_bytes = [0; 4];
    context.load_bytes(0u64, &mut stack_bytes)?;
    let stack_top = u32::from_le_bytes(stack_bytes);
    context.write_sp(stack_top, &dft::Tag::from(tag::ACCESSED))?;

    let mut entry_bytes = [0u8; 4];
    context.load_bytes(4u64, &mut entry_bytes)?;
    let entry = u32::from_le_bytes(entry_bytes);
    context.write_pc(entry, &dft::Tag::from(tag::ACCESSED))?;

    info!("building taint policy...");
    let callstack_plugin = CallStackPlugin::default();
    let call_channel = callstack_plugin.call_channel.clone();
    let lang = Arc::new(pdb.lang().clone());
    let policy = ttff::policy::TaintedAddressPolicy::new_with(lang, call_channel);

    info!("building evaluator...");
    let mut evaluator = dft::Evaluator::new_with_policy(Box::new(policy));
    evaluator.add_plugin(Box::new(callstack_plugin));
    (evaluator.pc, evaluator.pc_tag) = context.read_pc()
        .map(|(pc, tag)| (Location::from(pc), tag))?;

    info!("building dft executor...");
    let halt_on_exit = &mut |
        evaluator: &dft::Evaluator,
        _pdb: &ProgramDB,
        _context: &mut dft::Context,
    | {
        match evaluator.pc.address().offset() {
            // we can locate obvious exit functions statically as self loops.
            // a more sophisticated method would be to check if interrupts are
            // disabled and only halt then, but this sample has no interrupts.
            0xb74 => { info!("_exit reached"); Some(ExitKind::Ok) }
            _ => { None }
        }
    };
    let halt_cb = Some(sc::HaltCallback {
        callback: halt_on_exit,
    });

    let stop_on_policy_violation = &mut |
        result: &Result<(), dft::eval::Error>,
    | {
        match result {
            Err(dft::eval::Error::Policy(err)) => {
                // policy violation
                error!("policy violation: {err:#x?}");
                return Err(libafl::Error::ShuttingDown);
            }
            _ => { Ok(None) }
        }
    };
    let step_cb = Some(sc::StepCallback {
        callback: stop_on_policy_violation,
    });
    let post_exec_cb = None;

    let dft_executor = sc::DftExecutor::new_with(
        evaluator,
        context,
        pdb,
        limit,
        exc_limit,
        halt_cb,
        step_cb,
        post_exec_cb,
        access_log.clone(),
        rx_channel.clone(),
        tx_channel.clone(),
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
    let queue_corpus_path = PathBuf::from("examples/uart-address/queue");
    let crash_corpus_path = PathBuf::from("examples/uart-address/crashes");
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
    match state.generate_initial_inputs(
        &mut fuzzer, &mut executor, &mut generator, &mut manager, 8)
    {
        Err(libafl::Error::ShuttingDown) => { info!("fuzzer stopped by user."); return Ok(()) }
        Err(err) => { panic!("failed to generate initial corpus: {err:?}") }
        _ => {  }
    }

    match fuzzer.fuzz_loop(&mut stages, &mut executor, &mut state, &mut manager) {
        Err(libafl::Error::ShuttingDown) => { info!("fuzzer stopped by user."); Ok(()) }
        Err(err) => {
            error!("failed to run launcher: {err:?}");
            Err(err.into())
        }
        _ => { Ok(()) }
    }
}