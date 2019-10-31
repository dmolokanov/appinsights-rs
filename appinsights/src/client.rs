use crate::Config;

/// Application Insights telemetry client provides an interface to track telemetry items.
pub struct TelemetryClient {}

impl TelemetryClient {
    /// Creates a new telemetry client that submits telemetry with specified instrumentation key.
    pub fn new(instrumentation_key: String) -> Self {
        Self::from_config(Config::with_instrumentation_key(instrumentation_key))
    }

    pub fn from_config(config: Config) -> Self {
        Self {}
    }

    /// Logs a user action with the specified name.
    pub fn track_event(&self, name: &str) {
        println!("track_event: {}", name)
    }
}
