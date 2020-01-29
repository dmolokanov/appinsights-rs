use crate::telemetry::{ContextTags, Properties};
use crate::TelemetryConfig;

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
    /// Creates a new instance of telemetry context from config
    pub fn from_config(config: &TelemetryConfig) -> Self {
        let i_key = config.i_key().into();

        let sdk_version = format!("rust:{}", env!("CARGO_PKG_VERSION"));
        let os_version = if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            "unknown"
        };

        let mut tags = ContextTags::default();
        tags.internal_mut().set_sdk_version(sdk_version);
        tags.device_mut().set_os_version(os_version.into());
        // TODO get a hostname
        // TODO tags.device_mut().set_id(hostname.clone());
        // TODO tags.cloud_mut().set_role_instance(hostname);

        let properties = Properties::default();
        Self::new(i_key, tags, properties)
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
        let config = TelemetryConfig::new("instrumentation".into());
        let mut context = TelemetryContext::from_config(&config);
        context.properties_mut().insert("Resource Group".into(), "my-rg".into());

        assert_eq!(context.properties().len(), 1);
        assert_eq!(context.properties().get("Resource Group"), Some(&"my-rg".to_string()));
    }
}
