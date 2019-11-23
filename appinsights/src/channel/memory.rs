use std::thread;
use std::thread::JoinHandle;

use crossbeam_channel::{unbounded, Sender};
use log::{debug, trace, warn};

use crate::channel::command::Command;
use crate::channel::state::Worker;
use crate::channel::TelemetryChannel;
use crate::contracts::Envelope;
use crate::transmitter::Transmitter;
use crate::Config;
use crate::Result;

// A telemetry channel that stores events exclusively in memory.
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

        let worker = Worker::new(
            Transmitter::new(config.endpoint()),
            event_receiver,
            command_receiver,
            config.interval(),
        );

        let thread = thread::spawn(move || {
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
