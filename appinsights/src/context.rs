use crate::telemetry::{ContextTags, Properties};

/// Encapsulates contextual data common to all telemetry submitted through a telemetry client.
#[derive(Clone)]
pub struct TelemetryContext {
    /// An instrumentation key.
    pub(crate) i_key: String,

    // A stripped-down instrumentation key used in envelope name.
    pub(crate) normalized_i_key: String,

    // A collection of tags to attach to telemetry event.
    pub(crate) tags: ContextTags,

    // A collection of common properties to attach to telemetry event.
    pub(crate) properties: Properties,
}

impl TelemetryContext {
    /// Creates a new instance of telemetry context.
    pub fn new(i_key: String) -> Self {
        let normalized_i_key = i_key.replace("-", "");
        Self {
            i_key,
            normalized_i_key,
            tags: ContextTags::default(),
            properties: Properties::default(),
        }
    }

    /// Returns mutable reference to a collection of common properties to attach to telemetry event.
    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Returns mutable reference to a collection of common tags to attach to telemetry event.
    pub fn tags_mut(&mut self) -> &mut ContextTags {
        &mut self.tags
    }
}
