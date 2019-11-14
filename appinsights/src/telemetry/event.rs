use chrono::{DateTime, Utc};

use crate::context::TelemetryContext;
use crate::contracts::{Base, Data, Envelope, EnvelopeBuilder, EventDataBuilder};
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
    pub fn new(timestamp: DateTime<Utc>, name: &str) -> Self {
        Self {
            name: name.into(),
            timestamp,
            properties: Default::default(),
            tags: Default::default(),
            measurements: Default::default(),
        }
    }

    /// Returns custom measurements to submit with the telemetry item.
    pub fn measurements(&self) -> Option<&Measurements> {
        Some(&self.measurements)
    }

    /// Returns mutable reference to custom measurements.
    pub fn measurements_mut(&mut self) -> Option<&mut Measurements> {
        Some(&mut self.measurements)
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
        let data = Data::EventData(
            EventDataBuilder::new(telemetry.name)
                .properties(Properties::combine(context.properties, telemetry.properties))
                .measurements(telemetry.measurements)
                .build(),
        );

        let envelope_name = data.envelope_name(&context.normalized_i_key);

        EnvelopeBuilder::new(envelope_name, telemetry.timestamp.to_rfc3339())
            .data(Base::Data(data))
            .i_key(context.i_key)
            .tags(ContextTags::combine(context.tags, telemetry.tags))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::TimeZone;

    use super::*;

    #[test]
    fn it_overrides_properties_from_context() {
        let mut context = TelemetryContext::new("instrumentation".into());
        context.properties_mut().insert("test".into(), "ok".into());
        context.properties_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = EventTelemetry::new(Utc.ymd(2019, 1, 2).and_hms(3, 4, 5), "test".into());
        telemetry.properties_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.Event".into(),
            "2019-01-02T03:04:05+00:00".into(),
        )
        .data(Base::Data(Data::EventData(
            EventDataBuilder::new("test".into())
                .properties({
                    let mut properties = BTreeMap::default();
                    properties.insert("test".into(), "ok".into());
                    properties.insert("no-write".into(), "ok".into());
                    properties
                })
                .measurements(BTreeMap::default())
                .build(),
        )))
        .i_key("instrumentation".into())
        .tags(BTreeMap::default())
        .build();

        assert_eq!(envelop, expected)
    }

    #[test]
    fn it_overrides_tags_from_context() {
        let mut context = TelemetryContext::new("instrumentation".into());
        context.tags_mut().insert("test".into(), "ok".into());
        context.tags_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = EventTelemetry::new(Utc.ymd(2019, 1, 2).and_hms(3, 4, 5), "test".into());
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.Event".into(),
            "2019-01-02T03:04:05+00:00".into(),
        )
        .data(Base::Data(Data::EventData(
            EventDataBuilder::new("test".into())
                .properties(BTreeMap::default())
                .measurements(BTreeMap::default())
                .build(),
        )))
        .i_key("instrumentation".into())
        .tags({
            let mut tags = BTreeMap::default();
            tags.insert("test".into(), "ok".into());
            tags.insert("no-write".into(), "ok".into());
            tags
        })
        .build();

        assert_eq!(envelop, expected)
    }
}
