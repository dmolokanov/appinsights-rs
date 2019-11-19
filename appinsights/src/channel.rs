use crate::contracts::Envelope;
use crate::transmitter::Transmitter;
use crate::Config;
use crate::Result;
use crossbeam_channel::{after, select, unbounded, Receiver, Sender};
use log::{debug, error, info, trace};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

/// An implementation of [TelemetryChannel](trait.TelemetryChannel.html) is responsible for queueing
/// and periodically submitting telemetry events.
pub trait TelemetryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope) -> Result<()>;
}

/// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {
    event_sender: Sender<Envelope>,
    command_sender: Sender<Command>,
    thread: Option<JoinHandle<()>>,
}

impl InMemoryChannel {
    /// Creates a new instance of in-memory channel and starts a submission routine.
    pub fn new(config: &Config) -> Self {
        let (event_sender, event_receiver) = unbounded::<Envelope>();
        let (command_sender, command_receiver) = unbounded::<Command>();

        let mut worker = Worker {
            event_receiver,
            command_receiver,
            interval: config.interval(),
            transmitter: Transmitter::new(config.endpoint()),
            stopping: false,
        };

        let thread = thread::spawn(move || {
            while !worker.stopping {
                worker.run();
            }
        });

        Self {
            event_sender,
            command_sender,
            thread: Some(thread),
        }
    }
}

impl Drop for InMemoryChannel {
    fn drop(&mut self) {
        debug!("Sending terminate message to worker");
        self.command_sender.send(Command::Stop);

        debug!("Shutting down worker");
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Stop,
}

impl TelemetryChannel for InMemoryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope) -> Result<()> {
        Ok(self.event_sender.send(envelop)?)
    }
}

struct Worker {
    event_receiver: Receiver<Envelope>,
    command_receiver: Receiver<Command>,
    interval: Duration,
    transmitter: Transmitter,
    stopping: bool,
}

impl Worker {
    fn run(&mut self) {
        let mut items = Vec::<Envelope>::new();

        // delay until timeout passed
        let interval = after(self.interval);

        loop {
            select! {
                recv(self.event_receiver) -> envelope => {
                    match envelope {
                        Ok(envelope) => {
                            trace!("Event received");
                            items.push(envelope);
                        },
                        Err(err) => error!("Error occurred when reading events: {}", err),
                    }
                },
                recv(self.command_receiver) -> command => {
                    match command {
                        Ok(Command::Stop) => {
                            info!("Stop command received");
                            self.stopping = true;
                            break;
                        },
                        Ok(command) => panic!("Unsupported command received: {:?}", command),
                        Err(err) => error!("Error occurred when reading commands: {}", err),
                    }
                },
                recv(interval) -> _ => {
                    info!("Timeout expired");
                    break;
                }
            }
        }

        // send all messages collected so far to the server
        debug!("Sending {} events to the server", items.len());
        self.transmit(items)
    }

    fn transmit(&self, items: Vec<Envelope>) {
        let result = self.transmitter.transmit(&items);
        debug!("Transmission result: {:?}", result);
    }
}
