use crate::contracts::Envelope;
use crate::telemetry::Telemetry;
use crate::Config;

/// Encapsulates contextual data common to all telemetry submitted through a telemetry client.
pub struct TelemetryContext {
    /// Instrumentation key.
    ikey: String,
}

impl TelemetryContext {
    /// Wraps a telemetry event in an envelope with the information found in this context.
    pub fn envelop<T>(&self, event: T) -> Envelope
    where
        T: Telemetry,
    {
        Envelope {}
    }
}

impl From<&Config> for TelemetryContext {
    fn from(config: &Config) -> Self {
        Self {
            ikey: config.ikey().into(),
        }
    }
}
