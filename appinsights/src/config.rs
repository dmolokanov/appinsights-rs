/// Configuration data used to initialize a new [TelemetryClient](struct.TelemetryClient.html).
#[derive(Debug, PartialEq)]
pub struct Config {
    /// Instrumentation key for the client.
    instrumentation_key: String,

    /// Endpoint URL where data will be sent.
    endpoint: String,
}

impl Config {
    /// Creates a new configuration object with specified values.
    pub fn new(instrumentation_key: String, endpoint: String) -> Self {
        Self {
            instrumentation_key,
            endpoint,
        }
    }

    /// Creates a new configuration object with specified instrumentation key and default values.
    pub fn with_instrumentation_key(instrumentation_key: String) -> Self {
        Self::new(
            instrumentation_key,
            "https://dc.services.visualstudio.com/v2/track".into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_config_with_default_values() {
        let config = Config::with_instrumentation_key("instrumentation key".into());

        assert_eq!(
            Config {
                instrumentation_key: "instrumentation key".into(),
                endpoint: "https://dc.services.visualstudio.com/v2/track".into()
            },
            config
        )
    }
}
