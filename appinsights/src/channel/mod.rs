mod old;
mod state;

//pub use old::InMemoryChannel;
pub use state::InMemoryChannel;

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
    fn close(&self) -> Result<()>;
}

#[derive(Debug, PartialEq)]
pub enum Command {
    /// A command to tear down the submission, close internal channels. All pending telemetry items to be discarded.
    Stop,

    /// A command to force all pending telemetry items to be submitted.
    Flush,

    /// A command to tear down the submission, close internal channels and wait until all pending telemetry items to be sent.
    Close,
}
