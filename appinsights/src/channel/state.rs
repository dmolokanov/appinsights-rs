use crossbeam_channel::{after, select, unbounded, Receiver, Sender};
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::channel::TelemetryChannel;
use crate::contracts::Envelope;
use crate::transmitter::{Transmission, Transmitter};
use crate::Config;
use crate::Result;
use serde_json::map::Entry::Vacant;
use sm::{AsEnum, Event, Machine, NoneEvent, State, Transition};
use std::fmt::Display;
use std::net::Shutdown;

// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {
    sender: Sender<Command>,
    thread: Option<JoinHandle<()>>,
}

#[derive(Debug, Clone)]
enum Command {
    Event(Envelope),
    Flush,
    Terminate,
    Close,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Command::Event(_) => "send",
            Command::Flush => "flush",
            Command::Terminate => "terminate",
            Command::Close => "close",
        };
        write!(f, "{}", label)
    }
}

impl InMemoryChannel {
    /// Creates a new instance of in-memory channel and starts a submission routine.
    pub fn new(config: &Config) -> Self {
        let (sender, receiver) = unbounded::<Command>();
        let transmitter = Transmitter::new(config.endpoint());

        let thread = thread::spawn(move || {
            let mut worker = Worker::new().as_enum();
            let context = RunContext { transmitter, receiver };

            while !worker.is_stopped() {
                worker = worker.run(&context);
            }
        });

        Self {
            sender,
            thread: Some(thread),
        }
    }

    fn shutdown(&mut self, command: Command) {
        if let Some(thread) = self.thread.take() {
            debug!("Sending {} message to worker", command);
            if let Err(err) = self.sender.send(command.clone()) {
                warn!("Unable to send {} command: {}", command, err);
            }

            debug!("Shutting down worker");
            thread.join().unwrap();
        }
    }
}

impl Drop for InMemoryChannel {
    fn drop(&mut self) {
        self.shutdown(Command::Terminate);
    }
}

impl TelemetryChannel for InMemoryChannel {
    fn send(&self, envelop: Envelope) -> Result<()> {
        trace!("Sending item to channel");
        Ok(self.sender.send(Command::Event(envelop))?)
    }

    fn flush(&self) -> Result<()> {
        trace!("Sending flush command to channel");
        Ok(self.sender.send(Command::Flush)?)
    }

    fn close(&mut self) -> Result<()> {
        self.shutdown(Command::Close);
        Ok(())
    }
}

#[derive(Debug)]
struct Worker<S: State, E: Event> {
    state: S,
    event: Option<E>,
}

impl<S: State, E: Event> Machine for Worker<S, E> {
    type State = S;
    type Event = E;

    fn state(&self) -> Self::State {
        self.state.clone()
    }

    fn trigger(&self) -> Option<Self::Event> {
        self.event.clone()
    }
}

impl<S: State, E: Event> Eq for Worker<S, E> {}

impl<S: State, E: Event> PartialEq for Worker<S, E> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.event == other.event
    }
}

impl Worker<Receiving, NoneEvent> {
    pub fn new() -> Self {
        Self {
            state: Receiving::new(),
            event: None,
        }
    }
}

impl AsEnum for Worker<Receiving, NoneEvent> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::InitialReceiving(self)
    }
}

impl<E: Event> Run for Worker<Receiving, E> {
    fn run(self, context: &RunContext) -> Variant {
        select! {
            recv(context.receiver) -> command => {
                match command {
                    Ok(command) => match command {
                        Command::Event(envelope) => self.transition(ItemReceived(envelope)).as_enum(),
                        Command::Flush => self.transition(FlushRequested).as_enum(),
                        Command::Terminate => self.transition(StopRequested).as_enum(),
                        Command::Close => self.transition(CloseRequested).as_enum(),
                    },
                    Err(err) => panic!("commands channel closed: {}", err),
                }
            },
            recv(self.state.timer) -> _ => {
                info!("Timeout expired");
                self.transition(TimeoutExpired).as_enum()
            },
        }
    }
}

struct RunContext {
    transmitter: Transmitter,
    receiver: Receiver<Command>,
}

trait Run {
    fn run(self, context: &RunContext) -> Variant;
}

enum Variant {
    InitialReceiving(Worker<Receiving, NoneEvent>),
    ReceivingByReceiving(Worker<Receiving, ItemReceived>),
    ReceivingBySent(Worker<Receiving, ItemSent>),
    ReceivingByRetryExhausted(Worker<Receiving, RetryExhausted>),
    SendingByRetry(Worker<Sending, RetryRequested>),
    SendingByTimeout(Worker<Sending, TimeoutExpired>),
    SendingByFlush(Worker<Sending, FlushRequested>),
    SendingByClose(Worker<Sending, CloseRequested>),
    Stopped,
}

impl Variant {
    fn run(self, context: &RunContext) -> Variant {
        match self {
            Variant::InitialReceiving(w) => w.run(context),
            Variant::ReceivingByReceiving(w) => w.run(context),
            Variant::ReceivingBySent(w) => w.run(context),
            Variant::ReceivingByRetryExhausted(w) => w.run(context),
            Variant::SendingByRetry(w) => w.run(context),
            Variant::SendingByTimeout(w) => w.run(context),
            Variant::SendingByFlush(w) => w.run(context),
            Variant::SendingByClose(w) => w.run(context),
            Variant::Stopped => Variant::Stopped,
        }
    }

    fn is_stopped(&self) -> bool {
        match self {
            Variant::Stopped => true,
            _ => false,
        }
    }
}

impl<E: Event> Transition<ItemReceived> for Worker<Receiving, E> {
    type Machine = Worker<Receiving, ItemReceived>;

    fn transition(self, event: ItemReceived) -> Self::Machine {
        Worker {
            state: self.state.push(event.0.clone()), //clone here
            event: Some(event),
        }
    }
}

impl AsEnum for Worker<Receiving, ItemReceived> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::ReceivingByReceiving(self)
    }
}

impl<E: Event> Transition<StopRequested> for Worker<Receiving, E> {
    type Machine = Worker<Stopped, StopRequested>;

    fn transition(self, event: StopRequested) -> Self::Machine {
        Worker {
            state: Stopped,
            event: Some(event),
        }
    }
}

impl AsEnum for Worker<Stopped, StopRequested> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::Stopped
    }
}

impl<E: Event> Transition<FlushRequested> for Worker<Receiving, E> {
    type Machine = Worker<Sending, FlushRequested>;

    fn transition(self, event: FlushRequested) -> Self::Machine {
        Worker {
            state: Sending {
                items: self.state.items,
                retry: Retry::exponential(),
            },
            event: Some(event),
        }
    }
}

impl AsEnum for Worker<Sending, FlushRequested> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::SendingByFlush(self)
    }
}

//impl Run for Worker<Sending, FlushRequested> {
//    fn run(self, context: &RunContext) -> Variant {
//        debug!("Sending all pending items: {:?}", self.state.items.len());
//
//        self.transition(ItemSent).as_enum()
//    }
//}

impl<E: Event> Transition<CloseRequested> for Worker<Receiving, E> {
    type Machine = Worker<Sending, CloseRequested>;

    fn transition(self, event: CloseRequested) -> Self::Machine {
        Worker {
            state: Sending {
                items: self.state.items,
                retry: Retry::once().and_stop(),
            },
            event: Some(event),
        }
    }
}

impl AsEnum for Worker<Sending, CloseRequested> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::SendingByClose(self)
    }
}

//impl Run for Worker<Sending, CloseRequested> {
//    fn run(self, context: &RunContext) -> Variant {
//        debug!("Sending all pending items: {:?} and stop", self.state.items.len());
//
//        self.transition(ItemSent).as_enum()
//    }
//}

impl<E: Event> Transition<TimeoutExpired> for Worker<Receiving, E> {
    type Machine = Worker<Sending, TimeoutExpired>;

    fn transition(self, event: TimeoutExpired) -> Self::Machine {
        Self::Machine {
            state: Sending {
                items: self.state.items,
                retry: Retry::exponential(),
            },
            event: Some(event),
        }
    }
}

impl<E: Event> Run for Worker<Sending, E> {
    fn run(self, context: &RunContext) -> Variant {
        let Sending { items, retry } = self.state.clone(); // clone here
        debug!("{:?}: sending items: {:?}", self.event, items.len());

        if let Some(timeout) = retry.timeout() {
            // sleep until next sending attempt
            thread::sleep(timeout);

            // attempt to send items
            if let Ok(transmission) = context.transmitter.transmit(&items) {
                if retry.should_stop() {
                    return self.transition(StopRequested).as_enum();
                }

                if transmission.is_success() {
                    return self.transition(ItemSent).as_enum();
                }

                if transmission.can_retry() {
                    // make an attempt to re-send only if there are some items in the list
                    let items = transmission.retry_items(items);
                    if items.is_empty() {
                        return self.transition(ItemSent).as_enum();
                    }

                    return self.transition(RetryRequested(items, retry.next())).as_enum();
                }

                return self.transition(RetryExhausted).as_enum();
            }
        }

        if retry.should_stop() {
            return self.transition(StopRequested).as_enum();
        }

        self.transition(RetryExhausted).as_enum()
    }
}

impl AsEnum for Worker<Sending, TimeoutExpired> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::SendingByTimeout(self)
    }
}

impl<E: Event> Transition<RetryRequested> for Worker<Sending, E> {
    type Machine = Worker<Sending, RetryRequested>;

    fn transition(self, event: RetryRequested) -> Self::Machine {
        Self::Machine {
            event: Some(event.clone()),
            state: Sending {
                items: event.0,
                retry: event.1,
            },
        }
    }
}

impl AsEnum for Worker<Sending, RetryRequested> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::SendingByRetry(self)
    }
}

impl<E: Event> Transition<RetryExhausted> for Worker<Sending, E> {
    type Machine = Worker<Receiving, RetryExhausted>;

    fn transition(self, event: RetryExhausted) -> Self::Machine {
        Worker {
            state: Receiving::new(),
            event: Some(event),
        }
    }
}

impl AsEnum for Worker<Receiving, RetryExhausted> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::ReceivingByRetryExhausted(self)
    }
}

impl<E: Event> Transition<StopRequested> for Worker<Sending, E> {
    type Machine = Worker<Stopped, StopRequested>;

    fn transition(self, event: StopRequested) -> Self::Machine {
        Worker {
            state: Stopped,
            event: Some(event),
        }
    }
}

impl<E: Event> Transition<ItemSent> for Worker<Sending, E> {
    type Machine = Worker<Receiving, ItemSent>;

    fn transition(self, event: ItemSent) -> Self::Machine {
        Worker {
            state: Receiving::new(),
            event: Some(event),
        }
    }
}

impl AsEnum for Worker<Receiving, ItemSent> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::ReceivingBySent(self)
    }
}

impl Receiving {
    fn new() -> Self {
        Self {
            items: Default::default(),
            timer: after(Duration::from_secs(2)),
        }
    }

    fn push(self, item: Envelope) -> Self {
        let mut items = self.items;
        items.push(item);
        Self {
            items,
            timer: self.timer,
        }
    }
}
#[derive(Clone, Debug)]
struct Receiving {
    items: Vec<Envelope>,
    timer: Receiver<Instant>,
}
impl State for Receiving {}

impl PartialEq for Receiving {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl Eq for Receiving {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Sending {
    items: Vec<Envelope>,
    retry: Retry,
}
impl State for Sending {}

//#[derive(Debug, Clone, Eq, PartialEq)]
//struct Stopping;
//impl State for Stopping {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Stopped;
impl State for Stopped {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ItemReceived(Envelope);
impl Event for ItemReceived {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TimeoutExpired;
impl Event for TimeoutExpired {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct StopRequested;
impl Event for StopRequested {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ItemSent;
impl Event for ItemSent {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FlushRequested;
impl Event for FlushRequested {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CloseRequested;
impl Event for CloseRequested {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RetryExhausted;
impl Event for RetryExhausted {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RetryRequested(Vec<Envelope>, Retry);
impl Event for RetryRequested {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Retry(Vec<Duration>, bool);

impl Retry {
    pub fn exponential() -> Self {
        let timeouts = vec![Duration::from_secs(16), Duration::from_secs(4), Duration::from_secs(0)];
        Self(timeouts, false)
    }

    pub fn once() -> Self {
        let timeouts = vec![Duration::from_secs(0)];
        Self(timeouts, false)
    }

    pub fn and_stop(self) -> Self {
        Self(self.0, true)
    }

    pub fn timeout(&self) -> Option<Duration> {
        self.0.last().copied()
    }

    pub fn should_stop(&self) -> bool {
        self.1
    }

    pub fn next(self) -> Self {
        let mut timeouts = self.0;
        timeouts.pop();

        Retry(timeouts, self.1)
    }
}
