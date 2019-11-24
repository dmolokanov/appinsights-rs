use std::time::Duration;

/// Configuration data used to initialize a new [TelemetryClient](struct.TelemetryClient.html) with.
///
/// # Examples
///
/// Creating a telemetry client configuration with default settings
/// ```rust
/// # use appinsights::TelemetryConfig;
/// let config = TelemetryConfig::new("<instrumentation key>".into());
/// ```
///
/// Creating a telemetry client configuration with customg settings
/// ```rust
/// # use std::time::Duration;
/// # use appinsights::TelemetryConfig;
/// let config = TelemetryConfig::builder()
///     .i_key("<instrumentation key>")
///     .interval(Duration::from_secs(5))
///     .build();
/// ```
#[derive(Debug, PartialEq)]
pub struct TelemetryConfig {
    /// Instrumentation key for the client.
    i_key: String,

    /// Endpoint URL where data will be sent.
    endpoint: String,

    // Maximum time to wait until send a batch of telemetry.
    interval: Duration,
}

impl TelemetryConfig {
    /// Creates a new telemetry configuration with specified instrumentation key and default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use appinsights::TelemetryConfig;
    /// let config = TelemetryConfig::new("<instrumentation key>".into());
    ///
    /// assert_eq!(config.i_key(), "<instrumentation key>");
    /// assert_eq!(config.interval(), Duration::from_secs(2));
    /// assert_eq!(config.endpoint(), "https://dc.services.visualstudio.com/v2/track");
    /// ```
    pub fn new(i_key: String) -> Self {
        TelemetryConfig::builder().i_key(i_key).build()
    }

    /// Creates a new telemetry configuration builder with default parameters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use appinsights::TelemetryConfig;
    /// let config = TelemetryConfig::builder()
    ///     .i_key("<another instrumentation key>")
    ///     .interval(Duration::from_secs(5))
    ///     .build();
    ///
    /// assert_eq!(config.i_key(), "<another instrumentation key>");
    /// assert_eq!(config.interval(), Duration::from_secs(5));
    /// ```
    pub fn builder() -> DefaultTelemetryConfigBuilder {
        DefaultTelemetryConfigBuilder::default()
    }

    /// Returns an instrumentation key for the client.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use appinsights::TelemetryConfig;
    /// let config = TelemetryConfig::new("<instrumentation key>".into());
    ///
    /// assert_eq!(config.i_key(), "<instrumentation key>");
    /// ```
    pub fn i_key(&self) -> &str {
        &self.i_key
    }

    /// Returns endpoint URL where data will be sent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use appinsights::TelemetryConfig;
    /// let config = TelemetryConfig::new("<instrumentation key>".into());
    ///
    /// assert_eq!(config.endpoint(), "https://dc.services.visualstudio.com/v2/track");
    /// ```
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    // Returns maximum time to wait until send a batch of telemetry.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use appinsights::TelemetryConfig;
    /// let config = TelemetryConfig::new("<instrumentation key>".into());
    ///
    /// assert_eq!(config.interval(), Duration::from_secs(2));
    /// ```
    pub fn interval(&self) -> Duration {
        self.interval
    }
}

/// Constructs a new instance of a [TelemetryConfig](struct.TelemetryConfig.html) with required
/// instrumentation key and custom settings.
#[derive(Default)]
pub struct DefaultTelemetryConfigBuilder;

impl DefaultTelemetryConfigBuilder {
    /// Initializes a builder with an instrumentation key for the client.
    pub fn i_key<I>(self, i_key: I) -> TelemetryConfigBuilder
    where
        I: Into<String>,
    {
        TelemetryConfigBuilder {
            i_key: i_key.into(),
            endpoint: "https://dc.services.visualstudio.com/v2/track".into(),
            interval: Duration::from_secs(2),
        }
    }
}

/// Constructs a new instance of a [TelemetryConfig](struct.TelemetryConfig.html) with custom settings.
pub struct TelemetryConfigBuilder {
    i_key: String,
    endpoint: String,
    interval: Duration,
}

impl TelemetryConfigBuilder {
    /// Initializes a builder with an instrumentation key for the client.
    pub fn i_key<I>(mut self, i_key: I) -> Self
    where
        I: Into<String>,
    {
        self.i_key = i_key.into();
        self
    }

    /// Initializes a builder with an endpoint URL where data will be sent.
    pub fn endpoint<E>(mut self, endpoint: E) -> Self
    where
        E: Into<String>,
    {
        self.endpoint = endpoint.into();
        self
    }

    /// Initializes a builder with a maximum time to wait until send a batch of telemetry.
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    pub fn build(self) -> TelemetryConfig {
        TelemetryConfig {
            i_key: self.i_key,
            endpoint: self.endpoint,
            interval: self.interval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_config_with_default_values() {
        let config = TelemetryConfig::new("instrumentation key".into());

        assert_eq!(
            TelemetryConfig {
                i_key: "instrumentation key".into(),
                endpoint: "https://dc.services.visualstudio.com/v2/track".into(),
                interval: Duration::from_secs(2)
            },
            config
        )
    }

    #[test]
    fn it_builds_config_with_custom_parameters() {
        let config = TelemetryConfig::builder()
            .i_key("instrumentation key")
            .endpoint("https://google.com")
            .interval(Duration::from_micros(100))
            .build();

        assert_eq!(
            TelemetryConfig {
                i_key: "instrumentation key".into(),
                endpoint: "https://google.com".into(),
                interval: Duration::from_micros(100)
            },
            config
        );
    }
}
