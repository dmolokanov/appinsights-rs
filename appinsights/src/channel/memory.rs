use std::thread;
use std::thread::JoinHandle;

use crossbeam_channel::{unbounded, Sender};
use log::{debug, trace, warn};

use crate::channel::command::Command;
use crate::channel::state::Worker;
use crate::channel::TelemetryChannel;
use crate::contracts::Envelope;
use crate::transmitter::Transmitter;
use crate::TelemetryConfig;

/// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {
    event_sender: Sender<Envelope>,
    command_sender: Option<Sender<Command>>,
    thread: Option<JoinHandle<()>>,
}

impl InMemoryChannel {
    /// Creates a new instance of in-memory channel and starts a submission routine.
    pub fn new(config: &TelemetryConfig) -> Self {
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
            command_sender: Some(command_sender),
            thread: Some(thread),
        }
    }

    fn shutdown(&mut self, command: Command) {
        if let Some(sender) = self.command_sender.take() {
            Self::send_command(&sender, command);
        }

        if let Some(thread) = self.thread.take() {
            debug!("Shutting down worker");
            thread.join().unwrap();
        }
    }

    fn send_command(sender: &Sender<Command>, command: Command) {
        debug!("Sending {} command to channel", command);
        if let Err(err) = sender.send(command.clone()) {
            warn!("Unable to send {} command to channel: {}", command, err);
        }
    }
}

impl Drop for InMemoryChannel {
    fn drop(&mut self) {
        self.shutdown(Command::Terminate);
    }
}

impl TelemetryChannel for InMemoryChannel {
    fn send(&self, envelop: Envelope) {
        trace!("Sending telemetry to channel");
        if let Err(err) = self.event_sender.send(envelop) {
            warn!("Unable to send telemetry to channel: {}", err);
        }
    }

    fn flush(&self) {
        if let Some(sender) = &self.command_sender {
            Self::send_command(sender, Command::Flush);
        }
    }

    fn close(&mut self) {
        self.shutdown(Command::Close);
    }
}
