use std::time::Duration;

/// Configuration data used to initialize a new [TelemetryClient](struct.TelemetryClient.html).
#[derive(Debug, PartialEq)]
pub struct Config {
    /// Instrumentation key for the client.
    i_key: String,

    /// Endpoint URL where data will be sent.
    endpoint: String,

    // Maximum time to wait until send a batch of telemetry.
    interval: Duration,
}

impl Config {
    /// Creates a new configuration object with specified instrumentation key and default values.
    pub fn new(i_key: String) -> Self {
        Config::builder().with_i_key(i_key).build()
    }

    /// Creates a new configuration builder with default parameters.
    pub fn builder() -> DefaultBuilder {
        DefaultBuilder::default()
    }

    /// Returns an instrumentation key for the client.
    pub fn i_key(&self) -> &str {
        &self.i_key
    }

    /// Returns endpoint URL where data will be sent.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    // Returns maximum time to wait until send a batch of telemetry.
    pub fn interval(&self) -> Duration {
        self.interval
    }
}

#[derive(Default)]
pub struct DefaultBuilder;

impl DefaultBuilder {
    pub fn with_i_key<I>(self, i_key: I) -> Builder
    where
        I: Into<String>,
    {
        Builder {
            i_key: i_key.into(),
            endpoint: "https://dc.services.visualstudio.com/v2/track".into(),
            interval: Duration::from_secs(2),
        }
    }
}

pub struct Builder {
    i_key: String,
    endpoint: String,
    interval: Duration,
}

impl Builder {
    pub fn with_i_key<I>(mut self, i_key: I) -> Self
    where
        I: Into<String>,
    {
        self.i_key = i_key.into();
        self
    }

    pub fn with_endpoint<E>(mut self, endpoint: E) -> Self
    where
        E: Into<String>,
    {
        self.endpoint = endpoint.into();
        self
    }

    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    pub fn build(self) -> Config {
        Config {
            i_key: self.i_key,
            endpoint: self.endpoint,
            interval: self.interval,
        }
    }

    /// Returns endpoint URL where data will be sent.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    // Returns maximum time to wait until send a batch of telemetry.
    pub fn interval(&self) -> Duration {
        self.interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_config_with_default_values() {
        let config = Config::new("instrumentation key".into());

        assert_eq!(
            Config {
                i_key: "instrumentation key".into(),
                endpoint: "https://dc.services.visualstudio.com/v2/track".into(),
                interval: Duration::from_secs(2)
            },
            config
        )
    }

    #[test]
    fn it_builds_config_with_custom_parameters() {
        let config = Config::builder()
            .with_i_key("instrumentation key")
            .with_endpoint("https://google.com")
            .with_interval(Duration::from_micros(100))
            .build();

        assert_eq!(
            Config {
                i_key: "instrumentation key".into(),
                endpoint: "https://google.com".into(),
                interval: Duration::from_micros(100)
            },
            config
        );
    }
}
