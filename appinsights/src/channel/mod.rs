mod command;

mod memory;
pub use memory::InMemoryChannel;

mod retry;

mod state;

use async_trait::async_trait;

use crate::contracts::Envelope;

/// An implementation of [TelemetryChannel](trait.TelemetryChannel.html) is responsible for queueing
/// and periodically submitting telemetry events.
#[async_trait]
pub trait TelemetryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope);

    /// Forces all pending telemetry items to be submitted. The current task will not be blocked.
    fn flush(&self);

    /// Flushes and tears down the submission flow and closes internal channels.
    /// It blocks the current task until all pending telemetry items have been submitted and it is safe to
    /// shutdown without losing telemetry.
    async fn close(&mut self);

    /// Flushes and tears down the submission flow and closes internal channels.
    /// It blocks the current task until all pending telemetry items have been submitted and it is safe to
    /// shutdown without losing telemetry.
    /// Tears down the submission flow and closes internal channels. Any telemetry waiting to be sent is discarded.
    /// This is a more abrupt version of [close](#method.close).
    async fn terminate(&mut self);
}
