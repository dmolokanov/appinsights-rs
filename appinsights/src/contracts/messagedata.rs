use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Instances of Message represent printf-like trace statements that are text-searched. Log4Net, NLog and other text-based log file entries are translated into intances of this type. The message does not have measurements.
#[derive(Debug, Serialize)]
pub struct MessageData {
    ver: i32,
    message: String,
    severity_level: Option<SeverityLevel>,
    properties: Option<std::collections::HashMap<String, String>>,
    measurements: Option<std::collections::HashMap<String, f64>>,
}

impl MessageData {
    /// Create a new [MessageData](trait.MessageData.html) instance with default values set by the schema.
    pub fn new(message: String) -> Self {
        Self {
            ver: 2,
            message,
            severity_level: None,
            properties: None,
            measurements: None,
        }
    }

    /// Schema version
    pub fn with_ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Trace message
    pub fn with_message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }

    /// Trace severity level.
    pub fn with_severity_level(&mut self, severity_level: Option<SeverityLevel>) -> &mut Self {
        self.severity_level = severity_level;
        self
    }

    /// Collection of custom properties.
    pub fn with_properties(&mut self, properties: Option<std::collections::HashMap<String, String>>) -> &mut Self {
        self.properties = properties;
        self
    }

    /// Collection of custom measurements.
    pub fn with_measurements(&mut self, measurements: Option<std::collections::HashMap<String, f64>>) -> &mut Self {
        self.measurements = measurements;
        self
    }
}

impl TelemetryData for MessageData {
    /// Returns the base type when placed within an [Data](trait.Data.html) container.
    fn base_type(&self) -> String {
        String::from("MessageData")
    }
}
