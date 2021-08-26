mod command;

mod memory;
pub use memory::InMemoryChannel;

mod retry;

mod state;

use std::{future::Future, pin::Pin};

use crate::contracts::Envelope;

/// An implementation of [TelemetryChannel](trait.TelemetryChannel.html) is responsible for queueing
/// and periodically submitting telemetry events.
pub trait TelemetryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope);

    /// Forces all pending telemetry items to be submitted. The current thread will not be blocked.
    fn flush(&self);

    /// Flushes and tears down the submission flow and closes internal channels.
    /// It blocks current thread until all pending telemetry items have been submitted and it is safe to
    /// shutdown without losing telemetry.
    fn close(self) -> CloseFuture;
}

pub type CloseFuture = Pin<Box<dyn Future<Output = ()>>>;
