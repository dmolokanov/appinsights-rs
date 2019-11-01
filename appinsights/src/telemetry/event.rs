use crate::telemetry::Telemetry;

/// Represents structured event records.
pub struct EventTelemetry {
    /// Event name.
    name: String,
}

impl Telemetry for EventTelemetry {}

impl EventTelemetry {
    /// Creates an event telemetry item with specified name.
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}
