mod command;
mod memory;
mod retry;
mod state;

pub use memory::InMemoryChannel;

use crate::contracts::Envelope;
use crate::Result;

/// An implementation of [TelemetryChannel](trait.TelemetryChannel.html) is responsible for queueing
/// and periodically submitting telemetry events.
pub trait TelemetryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope) -> Result<()>;

    /// Forces all pending telemetry items to be submitted. The current thread will not be blocked.
    fn flush(&self) -> Result<()>;

    /// Flushes and tears down the submission flow and closes internal channels.
    /// It block current thread until all pending telemetry items have been submitted and it is safe to
    /// shutdown without losing telemetry.
    fn close(&mut self) -> Result<()>;
}
