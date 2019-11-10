use crate::contracts::TelemetryData;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Instances of Event represent structured event records that can be grouped and searched by their properties. Event data item also creates a metric of event count by name.
#[derive(Debug, Serialize)]
pub struct EventData {
    ver: i32,
    name: String,
    properties: Option<std::collections::HashMap<String, String>>,
    measurements: Option<std::collections::HashMap<String, f64>>,
}

impl EventData {
    /// Create a new [EventData](trait.EventData.html) instance with default values set by the schema.
    pub fn new(name: String) -> Self {
        Self {
            ver: 2,
            name,
            properties: None,
            measurements: None,
        }
    }

    /// Schema version
    pub fn with_ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Event name. Keep it low cardinality to allow proper grouping and useful metrics.
    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;
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

impl TelemetryData for EventData {}
