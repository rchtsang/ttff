//! sc.rs
//! 
//! single channel dft executor harness
use crossbeam::channel::{
    Receiver,
    Sender,
    TrySendError,
    // TryRecvError,
};
use libcme::{
    self,
    prelude::*,
    programdb::ProgramDB,
    peripheral::channel::Access,
};

use libafl::{
    executors::{Executor, ExitKind},
    state::{HasCorpus, HasExecutions},
    inputs::HasTargetBytes,
};

pub type HaltFn = dyn FnMut(
    &dft::Evaluator,
    &ProgramDB,
    &mut dft::Context,
) -> Option<ExitKind>;

/// a dft executor for channel-based peripherals
/// 
/// the base_context should be initialized at the point where
/// fuzzing should begin, so it must already be initialized for execution.
/// 
/// if cycle limit is None, then there is no limit.
pub struct DftExecutor<'policy, 'backend, 'irb, 'plugin> {
    /// an optional cycle count limit
    limit: Option<usize>,
    /// a halt condition callback
    halt_fn: Box<HaltFn>,
    evaluator: dft::Evaluator<'policy, 'plugin>,
    base_context: dft::Context<'backend>,
    pdb: ProgramDB<'irb>,
    access_log: (Sender<Access>, Receiver<Access>),
    read_src: (Sender<u8>, Receiver<u8>),
    write_dst: (Sender<u8>, Receiver<u8>),
}

impl<'policy, 'backend, 'irb, 'plugin> DftExecutor<'policy, 'backend, 'irb, 'plugin> {
    pub fn new_with(
        evaluator: dft::Evaluator<'policy, 'plugin>,
        base_context: dft::Context<'backend>,
        pdb: programdb::ProgramDB<'irb>,
        limit: Option<usize>,
        halt_fn: Option<Box<HaltFn>>,
        access_log: (Sender<Access>, Receiver<Access>),
        read_src: (Sender<u8>, Receiver<u8>),
        write_dst: (Sender<u8>, Receiver<u8>),
    ) -> Self {
        let halt_fn = halt_fn
            .unwrap_or(Box::new(|_, _, _| None));
        Self {
            evaluator,
            base_context,
            pdb,
            limit,
            halt_fn,
            access_log,
            read_src,
            write_dst,
        }
    }

    #[instrument(skip_all)]
    pub fn load_input<I>(&mut self, input: &I) -> Result<(), super::Error>
    where
        I: HasTargetBytes,
    {
        let slice = input.target_bytes();
        for (i, byte) in slice.iter().cloned().enumerate() {
            match self.read_src.0.try_send(byte) {
                Err(TrySendError::Disconnected(_)) => {
                    error!("failed to send byte #{i}: disconnected!");
                    return Err(super::Error::Input);
                }
                Err(TrySendError::Full(_)) => {
                    error!("failed to send byte #{i}: channel full!");
                    panic!("unbounded channel should never be full!");
                }
                _ => {  }
            }
        }
        Ok(())
    }
}


impl<'p, 'b, 'a, 'z, EM, I, S, Z> Executor<EM, I, S, Z> for DftExecutor<'p, 'b, 'a, 'z>
where
    S: HasCorpus<I> + HasExecutions,
    I: HasTargetBytes,
{
    #[instrument(skip_all)]
    fn run_target(
        &mut self,
        _fuzzer: &mut Z,
        state: &mut S,
        _mgr: &mut EM,
        input: &I,
    ) -> Result<ExitKind, libafl::Error> {
        // track execution count
        *state.executions_mut() += 1;

        let mut context = self.base_context.clone();

        // flush channels
        while let Ok(_access) = self.access_log.1.try_recv() {}
        while let Ok(_byte) = self.write_dst.1.try_recv() {}
        while let Ok(_byte) = self.read_src.1.try_recv() {}

        self.load_input(input)
            .map_err(|err| {
                libafl::Error::unknown(format!("{err:?}"))
            })?;

        let mut cycles: usize = 0;
        while self.limit.is_none() || cycles < self.limit.unwrap() {
            let result = self.evaluator.step(&mut context, &mut self.pdb);
            match result {
                Err(dft::eval::Error::Policy(err)) => {
                    // policy violation
                    error!("execution {:>4}: policy violation: {err:#x?}",
                        *state.executions());
                    return Ok(ExitKind::Crash);
                }
                Err(err) => {
                    // other evaluation/emulation error
                    error!("execution {:>4}: other error: {err:#x?}",
                        *state.executions());
                    return Ok(ExitKind::Crash);
                }
                _ => {
                    cycles += 1;
                    if let Some(kind) = (self.halt_fn)(
                        &mut self.evaluator, &mut self.pdb, &mut context)
                    {
                        return Ok(kind);
                    }
                }
            }
        }
        Ok(ExitKind::Timeout)
    }
}