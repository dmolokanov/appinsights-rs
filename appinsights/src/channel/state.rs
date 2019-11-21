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
use sm::{AsEnum, Event, Machine, NoneEvent, State, Transition};

// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {
    sender: Sender<Command>,
    thread: Option<JoinHandle<()>>,
}

#[derive(Debug)]
enum Command {
    Event(Envelope),
    Flush,
    Stop,
    Close,
}

impl InMemoryChannel {
    /// Creates a new instance of in-memory channel and starts a submission routine.
    pub fn new(config: &Config) -> Self {
        let (sender, receiver) = unbounded::<Command>();

        let thread = thread::spawn(move || {
            let mut worker = Worker::new(receiver).as_enum();

            while !worker.is_stopped() {
                worker = worker.run();
            }
        });

        Self {
            sender,
            thread: Some(thread),
        }
    }
}

impl Drop for InMemoryChannel {
    fn drop(&mut self) {
        debug!("Sending terminate message to worker");
        if let Err(err) = self.sender.send(Command::Stop) {
            warn!("Unable to send stop command: {}", err);
        }

        debug!("Shutting down worker");
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
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

    fn close(&self) -> Result<()> {
        trace!("Sending close command to channel");
        Ok(self.sender.send(Command::Close)?)
    }
}

#[derive(Debug)]
struct Worker<S: State, E: Event> {
    state: S,
    event: Option<E>,
    stopped: bool,
    receiver: Receiver<Command>,
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
    pub fn new(receiver: Receiver<Command>) -> Self {
        Self {
            state: Receiving::new(),
            event: None,
            stopped: false,
            receiver,
        }
    }
}

impl<E: Event> Run for Worker<Receiving, E> {
    fn run(self) -> Variant {
        select! {
            recv(self.receiver) -> command => {
                match command {
                    Ok(command) =>match command {
                        Command::Event(envelope) => self.transition(ItemReceived(envelope)).as_enum(),
                        Command::Flush => self.transition(FlushRequested).as_enum(),
                        Command::Stop => self.transition(StopRequested).as_enum(),
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

impl AsEnum for Worker<Receiving, NoneEvent> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::InitialReceiving(self)
    }
}

trait Run {
    fn run(self) -> Variant;
}

enum Variant {
    InitialReceiving(Worker<Receiving, NoneEvent>),
    ReceivingByReceiving(Worker<Receiving, ItemReceived>),
    ReceivingBySent(Worker<Receiving, ItemSent>),
    SendingByTimeout(Worker<Sending, TimeoutExpired>),
    SendingByFlush(Worker<Sending, FlushRequested>),
    SendingByClose(Worker<Sending, CloseRequested>),
    Stopped,
}

impl Variant {
    fn run(self) -> Variant {
        match self {
            Variant::InitialReceiving(w) => w.run(),
            Variant::ReceivingByReceiving(w) => w.run(),
            Variant::ReceivingBySent(w) => w.run(),
            Variant::SendingByTimeout(w) => w.run(),
            Variant::SendingByFlush(w) => w.run(),
            Variant::SendingByClose(w) => w.run(),
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
            stopped: false,
            receiver: self.receiver,
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
            stopped: false,
            receiver: self.receiver,
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
            },
            event: Some(event),
            stopped: false,
            receiver: self.receiver,
        }
    }
}

impl AsEnum for Worker<Sending, FlushRequested> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::SendingByFlush(self)
    }
}

impl Run for Worker<Sending, FlushRequested> {
    fn run(self) -> Variant {
        debug!("Sending all pending items: {:?}", self.state.items.len());

        self.transition(ItemSent).as_enum()
    }
}

impl<E: Event> Transition<CloseRequested> for Worker<Receiving, E> {
    type Machine = Worker<Sending, CloseRequested>;

    fn transition(self, event: CloseRequested) -> Self::Machine {
        Worker {
            state: Sending {
                items: self.state.items,
            },
            event: Some(event),
            stopped: false,
            receiver: self.receiver,
        }
    }
}

impl AsEnum for Worker<Sending, CloseRequested> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::SendingByClose(self)
    }
}

impl Run for Worker<Sending, CloseRequested> {
    fn run(self) -> Variant {
        debug!("Sending all pending items: {:?} and stop", self.state.items.len());

        self.transition(ItemSent).as_enum()
    }
}

impl<E: Event> Transition<TimeoutExpired> for Worker<Receiving, E> {
    type Machine = Worker<Sending, TimeoutExpired>;

    fn transition(self, event: TimeoutExpired) -> Self::Machine {
        Self::Machine {
            state: Sending {
                items: self.state.items,
            },
            event: Some(event),
            stopped: false,
            receiver: self.receiver,
        }
    }
}

impl Run for Worker<Sending, TimeoutExpired> {
    fn run(self) -> Variant {
        debug!("sending items: {:?}", self.state.items.len());
        self.transition(ItemSent).as_enum()
    }
}

impl AsEnum for Worker<Sending, TimeoutExpired> {
    type Enum = Variant;

    fn as_enum(self) -> Self::Enum {
        Variant::SendingByTimeout(self)
    }
}

impl<E: Event> Transition<ItemSent> for Worker<Sending, E> {
    type Machine = Worker<Receiving, ItemSent>;

    fn transition(self, event: ItemSent) -> Self::Machine {
        Worker {
            state: Receiving::new(),
            event: Some(event),
            stopped: false,
            receiver: self.receiver,
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
}
impl State for Sending {}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Stopping;
impl State for Stopping {}

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
