/// Configuration data used to initialize a new [TelemetryClient](struct.TelemetryClient.html).
#[derive(Debug, PartialEq)]
pub struct Config {
    /// Instrumentation key for the client.
    ikey: String,

    /// Endpoint URL where data will be sent.
    endpoint: String,
}

impl Config {
    /// Creates a new configuration object with specified values.
    pub fn new(ikey: String, endpoint: String) -> Self {
        Self { ikey, endpoint }
    }

    /// Creates a new configuration object with specified instrumentation key and default values.
    pub fn with_ikey(ikey: String) -> Self {
        Self::new(ikey, "https://dc.services.visualstudio.com/v2/track".into())
    }

    /// Returns an instrumentation key for the client.
    pub fn ikey(&self) -> &str {
        &self.ikey
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_config_with_default_values() {
        let config = Config::with_ikey("instrumentation key".into());

        assert_eq!(
            Config {
                ikey: "instrumentation key".into(),
                endpoint: "https://dc.services.visualstudio.com/v2/track".into()
            },
            config
        )
    }
}
