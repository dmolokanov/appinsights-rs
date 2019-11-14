use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Instances of Event represent structured event records that can be grouped and searched by their properties. Event data item also creates a metric of event count by name.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventData {
    ver: i32,
    name: String,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

/// Creates: Instances of Event represent structured event records that can be grouped and searched by their properties. Event data item also creates a metric of event count by name.
#[derive(Debug, Clone)]
pub struct EventDataBuilder {
    ver: i32,
    name: String,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl EventDataBuilder {
    /// Creates a new [EventDataBuilder](trait.EventDataBuilder.html) instance with default values set by the schema.
    pub fn new(name: String) -> Self {
        Self {
            ver: 2,
            name,
            properties: None,
            measurements: None,
        }
    }

    /// Sets: Schema version
    pub fn ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Sets: Collection of custom properties.
    pub fn properties(&mut self, properties: impl Into<std::collections::BTreeMap<String, String>>) -> &mut Self {
        self.properties = Some(properties.into());
        self
    }

    /// Sets: Collection of custom measurements.
    pub fn measurements(&mut self, measurements: impl Into<std::collections::BTreeMap<String, f64>>) -> &mut Self {
        self.measurements = Some(measurements.into());
        self
    }

    /// Creates a new [EventData](trait.EventData.html) instance with values from [EventDataBuilder](trait.EventDataBuilder.html).
    pub fn build(&self) -> EventData {
        EventData {
            ver: self.ver.clone(),
            name: self.name.clone(),
            properties: self.properties.clone(),
            measurements: self.measurements.clone(),
        }
    }
}
