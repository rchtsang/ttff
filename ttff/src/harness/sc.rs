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

pub type HaltCallbackFn = dyn FnMut(
    &dft::Evaluator,
    &ProgramDB,
    &mut dft::Context,
) -> Option<ExitKind>;

pub type StepCallbackFn = dyn FnMut(
    &Result<(), dft::eval::Error>,
) -> Result<Option<ExitKind>, libafl::Error>;

pub type PostExecCallbackFn = dyn FnMut(
    &mut dft::Evaluator,
    &mut ProgramDB,
    dft::Context,
    Result<ExitKind, libafl::Error>,
) -> Result<ExitKind, libafl::Error>;

pub struct HaltCallback<'a> {
    pub callback: &'a mut HaltCallbackFn,
}

pub struct StepCallback<'a> {
    pub callback: &'a mut StepCallbackFn,
}

pub struct PostExecCallback<'a> {
    pub callback: &'a mut PostExecCallbackFn,
}

/// a dft executor for channel-based peripherals
/// 
/// the base_context should be initialized at the point where
/// fuzzing should begin, so it must already be initialized for execution.
/// 
/// if cycle limit is None, then there is no limit.
pub struct DftExecutor<'policy, 'backend, 'irb, 'plugin> {
    /// an optional cycle count limit
    limit: Option<usize>,
    /// an optional maximum number of executions
    exc_limit: Option<usize>,
    /// a halt condition callback
    halt_cb: Option<HaltCallback<'plugin>>,
    /// a post-step callback
    /// (this is redundant and should be combined with halt_fn)
    step_cb: Option<StepCallback<'plugin>>,
    /// a post-execution callback
    post_exec_cb: Option<PostExecCallback<'plugin>>,
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
        exc_limit: Option<usize>,
        halt_cb: Option<HaltCallback<'plugin>>,
        step_cb: Option<StepCallback<'plugin>>,
        post_exec_cb: Option<PostExecCallback<'plugin>>,
        access_log: (Sender<Access>, Receiver<Access>),
        read_src: (Sender<u8>, Receiver<u8>),
        write_dst: (Sender<u8>, Receiver<u8>),
    ) -> Self {
        Self {
            evaluator,
            base_context,
            pdb,
            limit,
            exc_limit,
            halt_cb,
            step_cb,
            post_exec_cb,
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

    #[inline]
    fn post_exec(
        &mut self,
        context: dft::Context<'backend>,
        result: Result<ExitKind, libafl::Error>,
    ) -> Result<ExitKind, libafl::Error> {
        if let Some(ref mut post_exec_cb) = self.post_exec_cb {
            return (post_exec_cb.callback)(&mut self.evaluator, &mut self.pdb, context, result);
        }
        result
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
        if let Some(exc_limit) = self.exc_limit {
            if *state.executions() >= exc_limit as u64 {
                error!("execution limit hit! terminating fuzzer...");
                return Err(libafl::Error::ShuttingDown);
            }
        }

        // track execution count
        *state.executions_mut() += 1;
        info!("EXECUTION COUNT: {}", *state.executions());

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
            if let Some(ref mut step_cb) = self.step_cb {
                match (step_cb.callback)(&result) {
                    Ok(Some(kind)) => { return self.post_exec(context, Ok(kind)); }
                    Err(err) => { return self.post_exec(context, Err(err)); }
                    Ok(None) => {  }
                }
            }
            match result {
                Err(dft::eval::Error::Policy(err)) => {
                    // policy violation
                    error!("execution {:>4}: policy violation: {err:#x?}",
                        *state.executions());
                    return self.post_exec(context, Ok(ExitKind::Crash));
                }
                Err(dft::eval::Error::Context(
                    dft::context::Error::Backend(
                        backend::Error::Peripheral(err)
                ))) => {
                    let peripheral::Error::State(err) = err.as_ref() else {
                        return self.post_exec(context, Ok(ExitKind::Crash));
                    };
                    if let Some(peripheral::channel::ChannelStateError::Recv(addr, err)) = err.downcast_ref() {
                        error!("execution {:>4}: channel error on read at {}: {:?}",
                            *state.executions(),
                            addr.offset(),
                            err);
                        return self.post_exec(context, Ok(ExitKind::Timeout));
                    } else {
                        return self.post_exec(context, Ok(ExitKind::Crash));
                    }
                }
                Err(err) => {
                    // other evaluation/emulation error
                    error!("execution {:>4}: other error: {err:#x?}",
                        *state.executions());
                    return self.post_exec(context, Ok(ExitKind::Crash));
                }
                _ => {
                    cycles += 1;
                    if let Some(ref mut halt_cb) = self.halt_cb {
                        if let Some(kind) = (halt_cb.callback)(
                            &mut self.evaluator, &mut self.pdb, &mut context)
                        {
                            return self.post_exec(context, Ok(kind));
                        }
                    }
                }
            }
        }
        error!("cycle limit hit! exiting with timeout...");
        Ok(ExitKind::Timeout)
    }
}