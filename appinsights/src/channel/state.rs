use std::error::Error;
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crossbeam_channel::{after, select, unbounded, Receiver, Sender};
use log::{debug, error, info, trace, warn};
use sm::{sm, Event, State};

use crate::channel::TelemetryChannel;
use crate::contracts::Envelope;
use crate::transmitter::{Transmission, Transmitter};
use crate::Config;
use crate::Result;

use worker::{Variant::*, *};

// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {
    event_sender: Sender<Envelope>,
    command_sender: Sender<Command>,
    thread: Option<JoinHandle<()>>,
}

#[derive(Debug, Clone)]
enum Command {
    Flush,
    Terminate,
    Close,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
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
        let (event_sender, event_receiver) = unbounded::<Envelope>();
        let (command_sender, command_receiver) = unbounded::<Command>();
        let transmitter = Transmitter::new(config.endpoint());
        let interval = config.interval();

        let thread = thread::spawn(move || {
            let worker = Worker {
                transmitter,
                event_receiver,
                command_receiver,
                interval,
            };

            worker.run();
        });

        Self {
            event_sender,
            command_sender,
            thread: Some(thread),
        }
    }

    fn shutdown(&mut self, command: Command) {
        if let Some(thread) = self.thread.take() {
            debug!("Sending {} message to worker", command);
            if let Err(err) = self.command_sender.send(command.clone()) {
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
        Ok(self.event_sender.send(envelop)?)
    }

    fn flush(&self) -> Result<()> {
        trace!("Sending flush command to channel");
        Ok(self.command_sender.send(Command::Flush)?)
    }

    fn close(&mut self) -> Result<()> {
        self.shutdown(Command::Close);
        Ok(())
    }
}

sm! {
    worker {
        InitialStates { Receiving }

        TimeoutExpired {
            Receiving => Sending,
            Waiting => Sending
        }

        FlushRequested {
            Receiving => Sending
        }

        CloseRequested {
            Receiving => Sending,
            Waiting => Stopped
        }

        ItemsSentAndContinue {
            Sending => Receiving
        }

        ItemsSentAndStop {
            Sending => Stopped
        }

        RetryRequested {
            Sending => Waiting
        }

        RetryExhausted {
            Waiting => Receiving
        }

        TerminateRequested {
            Receiving => Stopped,
            Sending => Stopped,
            Waiting => Stopped
        }
    }
}

struct Worker {
    transmitter: Transmitter,
    event_receiver: Receiver<Envelope>,
    command_receiver: Receiver<Command>,
    interval: Duration,
}

impl Worker {
    pub fn run(&self) {
        let mut state = Machine::new(Receiving).as_enum();

        let mut items: Vec<Envelope> = Default::default();
        let mut retry = Retry::default();

        loop {
            state = match state {
                InitialReceiving(m) => self.handle_receiving(m, &mut items),
                ReceivingByItemsSentAndContinue(m) => self.handle_receiving(m, &mut items),
                ReceivingByRetryExhausted(m) => self.handle_receiving(m, &mut items),
                SendingByTimeoutExpired(m) => self.handle_sending_with_retry(m, &mut items, &mut retry),
                SendingByFlushRequested(m) => self.handle_sending_with_retry(m, &mut items, &mut retry),
                SendingByCloseRequested(m) => self.handle_sending_once_and_terminate(m, &mut items, &mut retry),
                WaitingByRetryRequested(m) => self.handle_waiting(m, &mut items, &mut retry),
                StoppedByItemsSentAndStop(_) => break,
                StoppedByCloseRequested(_) => break,
                StoppedByTerminateRequested(_) => break,
            }
        }
    }

    fn handle_receiving<E: Event>(&self, m: Machine<Receiving, E>, items: &mut Vec<Envelope>) -> Variant {
        debug!("Receiving messages triggered by {:?}", m.trigger());

        let timeout = after(self.interval);
        items.clear();

        loop {
            select! {
                recv(self.event_receiver) -> event => {
                    match event {
                        Ok(envelope) => {
                            items.push(envelope);
                            continue
                        },
                        Err(err) => {
                            error!("event channel closed: {}", err);
                            return m.transition(TerminateRequested).as_enum()
                        }
                    }
                }
                recv(self.command_receiver) -> command => {
                    match command {
                        Ok(command) => match command {
                            Command::Flush => return m.transition(FlushRequested).as_enum(),
                            Command::Terminate => return m.transition(TerminateRequested).as_enum(),
                            Command::Close => return m.transition(CloseRequested).as_enum(),
                        },
                        Err(err) => {
                            error!("commands channel closed: {}", err);
                            return m.transition(TerminateRequested).as_enum()
                        },
                    }
                },
                recv(timeout) -> _ => {
                    info!("Timeout expired");
                    return m.transition(TimeoutExpired).as_enum()
                },
            }
        }
    }

    fn handle_sending_with_retry<E: Event>(
        &self,
        m: Machine<Sending, E>,
        items: &mut Vec<Envelope>,
        retry: &mut Retry,
    ) -> Variant {
        *retry = Retry::exponential();
        self.handle_sending(m, items, retry)
    }

    fn handle_sending_once_and_terminate<E: Event>(
        &self,
        m: Machine<Sending, E>,
        items: &mut Vec<Envelope>,
        retry: &mut Retry,
    ) -> Variant {
        *retry = Retry::once();
        let cloned = m.clone(); // clone here
        self.handle_sending(m, items, retry);
        cloned.transition(TerminateRequested).as_enum()
    }

    fn handle_sending<E: Event>(
        &self,
        m: Machine<Sending, E>,
        items: &mut Vec<Envelope>,
        retry: &mut Retry,
    ) -> Variant {
        debug!(
            "Sending {} telemetry items triggered by {:?}",
            items.len(),
            m.trigger().unwrap()
        );

        // attempt to send items
        if let Ok(transmission) = self.transmitter.transmit(&items) {
            if transmission.is_success() {
                return m.transition(ItemsSentAndContinue).as_enum();
            }

            // make an attempt to re-send only if there are any items in the list that can be re-sent
            if transmission.can_retry() {
                *items = items
                    .drain(..)
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, envelope)| {
                        if transmission.can_retry_item(i) {
                            Some(envelope)
                        } else {
                            None
                        }
                    })
                    .collect();
                if items.is_empty() {
                    return m.transition(ItemsSentAndContinue).as_enum();
                }
            }
        }

        return m.transition(RetryRequested).as_enum();
    }

    fn handle_waiting<E: Event>(
        &self,
        m: Machine<Waiting, E>,
        items: &mut Vec<Envelope>,
        retry: &mut Retry,
    ) -> Variant {
        debug!(
            "Waiting for timeout {:?} or stop command triggered by {:?}",
            retry,
            m.state()
        );
        if let Some(timeout) = retry.next() {
            // sleep until next sending attempt
            let timeout = after(timeout);

            // wait for either timeout expired or stop command received
            loop {
                select! {
                    recv(self.command_receiver) -> command => {
                        match command {
                            Ok(command) => match command {
                                Command::Flush => continue,
                                Command::Terminate => return m.transition(TerminateRequested).as_enum(),
                                Command::Close => return m.transition(CloseRequested).as_enum(),
                            },
                            Err(err) => {
                                error!("commands channel closed: {}", err);
                                return m.transition(TerminateRequested).as_enum()
                            }
                        }
                    },
                    recv(timeout) -> _ => {
                        info!("Timeout expired");
                        return m.transition(TimeoutExpired).as_enum() // todo
                    },
                }
            }
        } else {
            return m.transition(RetryExhausted).as_enum();
        }
    }
}

#[derive(Default, Debug)]
struct Retry(Vec<Duration>);

impl Retry {
    pub fn exponential() -> Self {
        let timeouts = vec![Duration::from_secs(16), Duration::from_secs(4)];
        Self(timeouts)
    }

    pub fn once() -> Self {
        Self::default()
    }

    pub fn and_stop(self) -> Self {
        Self(self.0)
    }

    pub fn next(&mut self) -> Option<Duration> {
        self.0.pop()
    }
}
