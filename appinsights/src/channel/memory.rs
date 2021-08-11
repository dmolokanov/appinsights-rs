use log::{debug, trace, warn};
use tokio::{
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{
    channel::{command::Command, state::Worker, TelemetryChannel},
    contracts::Envelope,
    transmitter::Transmitter,
    TelemetryConfig,
};

/// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {
    event_sender: UnboundedSender<Envelope>,
    command_sender: Option<UnboundedSender<Command>>,
    join: Option<JoinHandle<()>>,
}

impl InMemoryChannel {
    /// Creates a new instance of in-memory channel and starts a submission routine.
    pub fn new(config: &TelemetryConfig) -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        let (command_sender, command_receiver) = mpsc::unbounded_channel();

        let worker = Worker::new(
            Transmitter::new(config.endpoint()),
            event_receiver,
            command_receiver,
            config.interval(),
        );

        let thread = tokio::spawn(worker.run());

        Self {
            event_sender,
            command_sender: Some(command_sender),
            join: Some(thread),
        }
    }

    fn shutdown(&mut self, command: Command) {
        if let Some(sender) = self.command_sender.take() {
            Self::send_command(&sender, command);
        }

        // TODO address this await

        // if let Some(thread) = self.join.take() {
        //     debug!("Shutting down worker");
        //     thread.await.unwrap();
        // }
    }

    fn send_command(sender: &UnboundedSender<Command>, command: Command) {
        debug!("Sending {} command to channel", command);
        if let Err(err) = sender.send(command.clone()) {
            warn!("Unable to send {} command to channel: {}", command, err);
        }
    }
}

// impl Drop for InMemoryChannel {
//     fn drop(&mut self) {
//         self.shutdown(Command::Terminate);
//     }
// }

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
