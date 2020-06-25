use std::thread;

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
    join_handle: ThreadHandle,
}

enum ThreadHandle {
    None,
    Thread(std::thread::JoinHandle<()>),
    Future(tokio::task::JoinHandle<()>),
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

        let future = async move { worker.run().await };

        let join_handle = if let Ok(handle) = tokio::runtime::Handle::try_current() {
            ThreadHandle::Future(handle.spawn(future))
        } else {
            ThreadHandle::Thread(thread::spawn(|| {
                let mut runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(future);
            }))
        };

        Self {
            event_sender,
            command_sender: Some(command_sender),
            join_handle,
        }
    }

    fn shutdown(&mut self, command: Command) {
        if let Some(sender) = self.command_sender.take() {
            Self::send_command(&sender, command);
        }

        match std::mem::replace(&mut self.join_handle, ThreadHandle::None) {
            ThreadHandle::Thread(thread) => thread.join().unwrap(),
            ThreadHandle::Future(_thread) => {} // TODO: can we block on this?
            ThreadHandle::None => {}
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
