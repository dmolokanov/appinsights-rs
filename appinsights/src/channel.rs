use crate::contracts::Envelope;
use crate::Config;

/// An implementation of [TelemetryChannel](trait.TelemetryChannel.html) is responsible for queueing
/// and periodically submitting telemetry events.
pub trait TelemetryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope);
}

/// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {}

impl InMemoryChannel {
    /// Creates a new instance of in-memory channel and starts a submission routine.
    pub fn new(config: &Config) -> Self {
        Self {}
    }
}

impl TelemetryChannel for InMemoryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope) {
        unimplemented!()
    }
}
