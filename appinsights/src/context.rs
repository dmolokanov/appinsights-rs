use crate::contracts::{Data, Envelope};
use crate::telemetry::Telemetry;
use crate::Config;

/// Encapsulates contextual data common to all telemetry submitted through a telemetry client.
pub struct TelemetryContext {
    /// Instrumentation key.
    i_key: String,
}

impl TelemetryContext {
    /// Wraps a telemetry event in an envelope with the information found in this context.
    pub fn envelop<T>(&self, event: T) -> Envelope
    where
        T: Telemetry,
        T::Data: From<T>,
    {
        // todo apply common properties
        let telemetry_data: T::Data = event.into();
        let mut data = Data::new(telemetry_data);
        // todo implement inheritance Base for Data

        let mut envelope = Envelope::new("test".into(), "time".into());
        envelope.with_data(None).with_flags(None);
        envelope
    }
}

impl From<&Config> for TelemetryContext {
    fn from(config: &Config) -> Self {
        Self {
            i_key: config.i_key().into(),
        }
    }
}
