use std::sync::Arc;

use async_trait::async_trait;
use crossbeam_queue::SegQueue;
use futures_channel::mpsc::UnboundedSender;
use log::{debug, trace, warn};
use tokio::task::JoinHandle;

use crate::{
    channel::{command::Command, state::Worker, TelemetryChannel},
    contracts::Envelope,
    transmitter::Transmitter,
    TelemetryConfig,
};

/// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {
    items: Arc<SegQueue<Envelope>>,
    command_sender: Option<UnboundedSender<Command>>,
    join: Option<JoinHandle<()>>,
}

impl InMemoryChannel {
    /// Creates a new instance of in-memory channel and starts a submission routine.
    pub fn new(config: &TelemetryConfig) -> Self {
        let items = Arc::new(SegQueue::new());

        let (command_sender, command_receiver) = futures_channel::mpsc::unbounded();
        let worker = Worker::new(
            Transmitter::new(config.endpoint()),
            items.clone(),
            command_receiver,
            config.interval(),
        );

        let handle = tokio::spawn(worker.run());

        Self {
            items,
            command_sender: Some(command_sender),
            join: Some(handle),
        }
    }

    async fn shutdown(mut self, command: Command) {
        // send shutdown command
        if let Some(sender) = self.command_sender.take() {
            send_command(&sender, command);
        }

        // wait until worker is finished
        if let Some(handle) = self.join.take() {
            debug!("Shutting down worker");
            handle.await.unwrap();
        }
    }
}

#[async_trait]
impl TelemetryChannel for InMemoryChannel {
    fn send(&self, envelop: Envelope) {
        trace!("Sending telemetry to channel");
        self.items.push(envelop);
    }

    fn flush(&self) {
        if let Some(sender) = &self.command_sender {
            send_command(sender, Command::Flush);
        }
    }

    async fn close(self) {
        self.shutdown(Command::Close).await
    }

    async fn terminate(self) {
        self.shutdown(Command::Terminate).await;
    }
}

fn send_command(sender: &UnboundedSender<Command>, command: Command) {
    debug!("Sending {} command to channel", command);
    if let Err(err) = sender.unbounded_send(command.clone()) {
        warn!("Unable to send {} command to channel: {}", command, err);
    }
}
