use chrono::{DateTime, Utc};

use crate::contracts::EventData;
use crate::telemetry::{ContextTags, Measurements, Properties, Telemetry};

/// Represents structured event records.
pub struct EventTelemetry {
    /// Event name.
    name: String,

    /// The time stamp when this telemetry was measured.
    timestamp: DateTime<Utc>,

    /// Custom properties.
    properties: Properties,

    /// Telemetry context containing extra, optional tags.
    tags: ContextTags,

    /// Custom measurements.
    measurements: Measurements,
}

impl EventTelemetry {
    /// Creates an event telemetry item with specified name.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            timestamp: Utc::now(),
            properties: Default::default(),
            tags: Default::default(),
            measurements: Default::default(),
        }
    }
}

impl Telemetry for EventTelemetry {
    /// Returns the time when this telemetry was measured.
    fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Returns custom properties to submit with the telemetry item.
    fn properties(&self) -> &Properties {
        &self.properties
    }

    /// Returns custom measurements to submit with the telemetry item.
    fn measurements(&self) -> Option<&Measurements> {
        Some(&self.measurements)
    }

    /// Returns context data containing extra, optional tags. Overrides values found on client telemetry context.
    fn tags(&self) -> &ContextTags {
        &self.tags
    }
}

impl From<EventTelemetry> for EventData {
    fn from(telemetry: EventTelemetry) -> Self {
        let mut data = EventData::new(telemetry.name);
        data.with_properties(telemetry.properties.into())
            .with_measurements(telemetry.measurements.into());

        data
    }
}
