use chrono::{DateTime, SecondsFormat, Utc};

use crate::context::TelemetryContext;
use crate::contracts::*;
use crate::telemetry::{ContextTags, Measurements, Properties, Telemetry};
use crate::time;

/// Represents structured event records.
///
/// # Examples
/// ```rust, no_run
/// # use appinsights::TelemetryClient;
/// # let client = TelemetryClient::new("<instrumentation key>".to_string());
/// use appinsights::telemetry::{Telemetry, EventTelemetry};
///
/// // create a telemetry item
/// let mut telemetry = EventTelemetry::new("Starting data processing".to_string());
///
/// // attach custom properties, measurements and context tags
/// telemetry.properties_mut().insert("component".to_string(), "data_processor".to_string());
/// telemetry.tags_mut().insert("os_version".to_string(), "linux x86_64".to_string());
/// telemetry.measurements_mut().insert("records_count".to_string(), 115.0);
///
/// // submit telemetry item to server
/// client.track(telemetry);
/// ```
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
    pub fn new(name: String) -> Self {
        Self {
            name,
            timestamp: time::now(),
            properties: Default::default(),
            tags: Default::default(),
            measurements: Default::default(),
        }
    }

    /// Returns custom measurements to submit with the telemetry item.
    pub fn measurements(&self) -> &Measurements {
        &self.measurements
    }

    /// Returns mutable reference to custom measurements.
    pub fn measurements_mut(&mut self) -> &mut Measurements {
        &mut self.measurements
    }
}

impl Telemetry for EventTelemetry {
    /// Returns the time when this telemetry was measured.
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// Returns custom properties to submit with the telemetry item.
    fn properties(&self) -> &Properties {
        &self.properties
    }

    /// Returns mutable reference to custom properties.
    fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Returns context data containing extra, optional tags. Overrides values found on client telemetry context.
    fn tags(&self) -> &ContextTags {
        &self.tags
    }

    /// Returns mutable reference to custom tags.
    fn tags_mut(&mut self) -> &mut ContextTags {
        &mut self.tags
    }
}

impl From<(TelemetryContext, EventTelemetry)> for Envelope {
    fn from((context, telemetry): (TelemetryContext, EventTelemetry)) -> Self {
        Envelope {
            name: "Microsoft.ApplicationInsights.Event".into(),
            time: telemetry.timestamp.to_rfc3339_opts(SecondsFormat::Millis, true),
            i_key: Some(context.i_key),
            tags: Some(ContextTags::combine(context.tags, telemetry.tags).into()),
            data: Some(Base::Data(Data::EventData(EventData {
                name: telemetry.name,
                properties: Some(Properties::combine(context.properties, telemetry.properties).into()),
                measurements: Some(telemetry.measurements.into()),
                ..EventData::default()
            }))),
            ..Envelope::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::TimeZone;

    use super::*;
    use crate::time;

    #[test]
    fn it_overrides_properties_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 600));

        let mut context = TelemetryContext::new("instrumentation".into());
        context.properties_mut().insert("test".into(), "ok".into());
        context.properties_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = EventTelemetry::new("test".into());
        telemetry.properties_mut().insert("no-write".into(), "ok".into());
        telemetry.measurements_mut().insert("value".into(), 5.0);

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.Event".into(),
            time: "2019-01-02T03:04:05.600Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some(BTreeMap::default()),
            data: Some(Base::Data(Data::EventData(EventData {
                name: "test".into(),
                properties: Some({
                    let mut properties = BTreeMap::default();
                    properties.insert("test".into(), "ok".into());
                    properties.insert("no-write".into(), "ok".into());
                    properties
                }),
                measurements: Some({
                    let mut measurements = BTreeMap::default();
                    measurements.insert("value".into(), 5.0);
                    measurements
                }),
                ..EventData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }

    #[test]
    fn it_overrides_tags_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 700));

        let mut context = TelemetryContext::new("instrumentation".into());
        context.tags_mut().insert("test".into(), "ok".into());
        context.tags_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = EventTelemetry::new("test".into());
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.Event".into(),
            time: "2019-01-02T03:04:05.700Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some({
                let mut tags = BTreeMap::default();
                tags.insert("test".into(), "ok".into());
                tags.insert("no-write".into(), "ok".into());
                tags
            }),
            data: Some(Base::Data(Data::EventData(EventData {
                name: "test".into(),
                properties: Some(BTreeMap::default()),
                measurements: Some(BTreeMap::default()),
                ..EventData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }
}
