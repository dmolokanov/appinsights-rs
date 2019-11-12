use crate::contracts::{Base, Data, Envelope, EnvelopeBuilder};
use crate::telemetry::{ContextTags, Telemetry};
use crate::Config;

/// Encapsulates contextual data common to all telemetry submitted through a telemetry client.
pub struct TelemetryContext {
    /// An instrumentation key.
    i_key: String,

    // A stripped-down instrumentation key used in envelope name.
    normalized_i_key: String,

    // A collection of tags to attach to telemetry event.
    tags: ContextTags,
}

impl TelemetryContext {
    /// Wraps a telemetry event in an envelope with the information found in this context.
    pub fn envelop<T>(&self, event: T) -> Envelope
    where
        T: Telemetry + Into<Data>,
    {
        // todo apply common properties

        // collect all tags from telemetry even; it can override some tags with default values found in context
        let mut tags = event.tags().clone();
        for (name, tag) in self.tags.clone() {
            tags.insert(name, tag);
        }

        let timestamp = event.timestamp().to_rfc3339();

        let data: Data = event.into();
        let envelope_name = data.envelope_name(&self.normalized_i_key);

        EnvelopeBuilder::new(envelope_name, timestamp)
            .data(Base::Data(data))
            .i_key(self.i_key.clone())
            .tags(tags)
            .build()
    }
}

// todo by rust guidelines impl From should consume specified value
impl From<&Config> for TelemetryContext {
    fn from(config: &Config) -> Self {
        Self {
            i_key: config.i_key().into(),
            normalized_i_key: config.i_key().replace("-", ""),
            tags: ContextTags::default(),
        }
    }
}
