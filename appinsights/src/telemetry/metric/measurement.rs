use chrono::{DateTime, SecondsFormat, Utc};

use crate::context::TelemetryContext;
use crate::contracts::*;
use crate::telemetry::{ContextTags, Properties, Telemetry};
use crate::time;

/// Metric telemetry item that represents a single data point.
pub struct MetricTelemetry {
    /// Metric name.
    name: String,

    /// Sampled value.
    value: f64,

    /// The time stamp when this telemetry was measured.
    timestamp: DateTime<Utc>,

    /// Custom properties.
    properties: Properties,

    /// Telemetry context containing extra, optional tags.
    tags: ContextTags,
}

impl MetricTelemetry {
    /// Creates a metric telemetry item with specified name and value.
    pub fn new(name: &str, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
            timestamp: time::now(),
            properties: Default::default(),
            tags: Default::default(),
        }
    }
}

impl Telemetry for MetricTelemetry {
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

impl From<(TelemetryContext, MetricTelemetry)> for Envelope {
    fn from((context, telemetry): (TelemetryContext, MetricTelemetry)) -> Self {
        let data_point = DataPointBuilder::new(telemetry.name, telemetry.value)
            .count(1)
            .kind(DataPointType::Measurement)
            .build();

        let data = Data::MetricData(
            MetricDataBuilder::new(data_point)
                .properties(Properties::combine(context.properties, telemetry.properties))
                .build(),
        );

        let envelope_name = data.envelope_name(&context.normalized_i_key);
        let timestamp = telemetry.timestamp.to_rfc3339_opts(SecondsFormat::Millis, true);

        EnvelopeBuilder::new(envelope_name, timestamp)
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
    use crate::time;

    #[test]
    fn it_overrides_properties_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 100));

        let mut context = TelemetryContext::new("instrumentation".into());
        context.properties_mut().insert("test".into(), "ok".into());
        context.properties_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = MetricTelemetry::new("test".into(), 123.0);
        telemetry.properties_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.Metric",
            "2019-01-02T03:04:05.100Z",
        )
        .data(Base::Data(Data::MetricData(
            MetricDataBuilder::new(
                DataPointBuilder::new("test", 123.0)
                    .count(1)
                    .kind(DataPointType::Measurement)
                    .build(),
            )
            .properties({
                let mut properties = BTreeMap::default();
                properties.insert("test".into(), "ok".into());
                properties.insert("no-write".into(), "ok".into());
                properties
            })
            .build(),
        )))
        .i_key("instrumentation")
        .tags(BTreeMap::default())
        .build();

        assert_eq!(envelop, expected)
    }

    #[test]
    fn it_overrides_tags_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 101));

        let mut context = TelemetryContext::new("instrumentation".into());
        context.tags_mut().insert("test".into(), "ok".into());
        context.tags_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = MetricTelemetry::new("test".into(), 123.0);
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.Metric",
            "2019-01-02T03:04:05.101Z",
        )
        .data(Base::Data(Data::MetricData(
            MetricDataBuilder::new(
                DataPointBuilder::new("test", 123.0)
                    .count(1)
                    .kind(DataPointType::Measurement)
                    .build(),
            )
            .properties(Properties::default())
            .build(),
        )))
        .i_key("instrumentation")
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
