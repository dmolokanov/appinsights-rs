use crate::contracts::Envelope;

/// An implementation of [TelemetryChannel](trait.TelemetryChannel.html) is responsible for queueing
/// and periodically submitting telemetry events.
pub trait TelemetryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope);
}

/// A telemetry channel that stores events exclusively in memory.
pub struct InMemoryChannel {}

impl TelemetryChannel for InMemoryChannel {
    /// Queues a single telemetry item.
    fn send(&self, envelop: Envelope) {
        unimplemented!()
    }
}
