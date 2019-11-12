use crate::contracts::{Envelope, EnvelopeBuilder, TelemetryData};
use crate::telemetry::Telemetry;
use crate::Config;

/// Encapsulates contextual data common to all telemetry submitted through a telemetry client.
pub struct TelemetryContext {
    /// Instrumentation key.
    i_key: String,

    // Stripped-down instrumentation key used in envelope name.
    normalized_i_key: String,
}

impl TelemetryContext {
    /// Wraps a telemetry event in an envelope with the information found in this context.
    pub fn envelop<T>(&self, event: T) -> Envelope
    where
        T: Telemetry,
        T::Data: From<T> + Clone,
    {
        let timestamp = event.timestamp();

        // todo apply common properties
        let telemetry_data: T::Data = event.into();
        //        let data = DataBuilder::new(telemetry_data).base_type(Some(telemetry_data.base_type()));
        // todo implement inheritance Base for Data

        EnvelopeBuilder::new(
            telemetry_data.envelope_name(&self.normalized_i_key),
            timestamp.to_rfc3339(),
        )
        .build()
    }
}

// todo by rust guidelines impl From should consume specified value
impl From<&Config> for TelemetryContext {
    fn from(config: &Config) -> Self {
        Self {
            i_key: config.i_key().into(),
            normalized_i_key: config.i_key().replace("-", ""),
        }
    }
}
