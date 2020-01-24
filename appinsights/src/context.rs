use crate::telemetry::{ContextTags, Properties};

/// Encapsulates contextual data common to all telemetry submitted through a telemetry client.
/// # Examples
/// ```rust
/// use appinsights::telemetry::{ContextTags, Properties};
/// use appinsights::TelemetryContext;
///
/// let mut properties = Properties::default();
/// properties.insert("Resource Group".to_string(), "my-rg".to_string());
///
/// let mut tags = ContextTags::default();
/// tags.insert("account_id".to_string(), "123-345-777".to_string());
///
/// let context = TelemetryContext::new("instrumentation".to_string(), tags, properties);
///
/// assert_eq!(context.properties().get("Resource Group"), Some(&"my-rg".to_string()));
/// assert_eq!(context.tags().get("account_id"), Some(&"123-345-777".to_string()));
/// ```
#[derive(Clone)]
pub struct TelemetryContext {
    /// An instrumentation key.
    pub(crate) i_key: String,

    // A collection of tags to attach to telemetry event.
    pub(crate) tags: ContextTags,

    // A collection of common properties to attach to telemetry event.
    pub(crate) properties: Properties,
}

impl TelemetryContext {
    /// Creates a new instance of telemetry context with instrumentation key and default tags and properties.
    pub fn with_i_key(i_key: String) -> Self {
        Self::new(i_key, ContextTags::default(), Properties::default())
    }

    /// Creates a new instance of telemetry context.
    pub fn new(i_key: String, tags: ContextTags, properties: Properties) -> Self {
        Self {
            i_key,
            tags,
            properties,
        }
    }

    /// Returns mutable reference to a collection of common properties to attach to telemetry event.
    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Returns immutable reference to a collection of common properties to attach to telemetry event.
    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    /// Returns mutable reference to a collection of common tags to attach to telemetry event.
    pub fn tags_mut(&mut self) -> &mut ContextTags {
        &mut self.tags
    }

    /// Returns immutable reference to a collection of common tags to attach to telemetry event.
    pub fn tags(&self) -> &ContextTags {
        &self.tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_updates_common_properties() {
        let mut context = TelemetryContext::with_i_key("intrumentation".to_string());
        context.properties_mut().insert("Resource Group".into(), "my-rg".into());

        assert_eq!(context.properties().len(), 1);
        assert_eq!(context.properties().get("Resource Group"), Some(&"my-rg".to_string()));
    }
}
