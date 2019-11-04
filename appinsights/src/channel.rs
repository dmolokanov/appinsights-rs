use crate::contracts::Envelope;
use crate::Config;
use crate::Result;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

/// An implementation of [TelemetryChannel](trait.TelemetryChannel.html) is responsible for queueing
/// and periodically submitting telemetry events.
pub trait TelemetryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope) -> Result<()>;
}

/// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {
    sender: Sender<Envelope>,
}

impl InMemoryChannel {
    /// Creates a new instance of in-memory channel and starts a submission routine.
    pub fn new(config: &Config) -> Self {
        let (sender, receiver) = mpsc::channel::<Envelope>();

        let worker = Worker {
            receiver,
            interval: config.interval(),
        };

        thread::spawn(move || {
            worker.run();
        });

        Self { sender }
    }
}

impl TelemetryChannel for InMemoryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope) -> Result<()> {
        Ok(self.sender.send(envelop)?)
    }
}

struct Worker {
    receiver: Receiver<Envelope>,
    interval: Duration,
}

impl Worker {
    fn run(&self) {
        loop {
            // read all messages from a channel
            let items = self.receiver.try_iter().collect();

            // transmit all messages to the server
            self.transmit(items);

            // wait until sending interval will expire
            thread::sleep(self.interval)
        }
    }

    fn transmit(&self, items: Vec<Envelope>) {
        for item in items {
            println!("Send item: {:?}", item);
        }
    }
}
