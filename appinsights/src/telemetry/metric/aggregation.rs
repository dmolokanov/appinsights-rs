use chrono::{DateTime, SecondsFormat, Utc};

use crate::context::TelemetryContext;
use crate::contracts::*;
use crate::telemetry::{ContextTags, Properties, Stats, Telemetry};
use crate::SystemTime;

/// Aggregated metric telemetry item that represents an aggregation of data points over time.
/// There values can be calculated by the caller or with add_data function.
pub struct AggregateMetricTelemetry {
    /// Metric name.
    name: String,

    /// Aggregated values stats.
    stats: Stats,

    /// The time stamp when this telemetry was measured.
    timestamp: DateTime<Utc>,

    /// Custom properties.
    properties: Properties,

    /// Telemetry context containing extra, optional tags.
    tags: ContextTags,
}

impl AggregateMetricTelemetry {
    /// Creates a metric telemetry item with specified name and value.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            stats: Stats::default(),
            timestamp: SystemTime::now(),
            properties: Default::default(),
            tags: Default::default(),
        }
    }

    /// Returns aggregated metric to submit with the telemetry item.
    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    /// Returns mutable reference to aggregated metric.
    pub fn stats_mut(&mut self) -> &mut Stats {
        &mut self.stats
    }
}

impl Telemetry for AggregateMetricTelemetry {
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

impl From<(TelemetryContext, AggregateMetricTelemetry)> for Envelope {
    fn from((context, telemetry): (TelemetryContext, AggregateMetricTelemetry)) -> Self {
        let data_point = DataPointBuilder::new(telemetry.name, telemetry.stats.value)
            .count(telemetry.stats.count)
            .kind(DataPointType::Aggregation)
            .min(telemetry.stats.min)
            .max(telemetry.stats.max)
            .std_dev(telemetry.stats.std_dev)
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
    use crate::SystemTime;

    #[test]
    fn it_overrides_properties_from_context() {
        SystemTime::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 100));

        let mut context = TelemetryContext::new("instrumentation".into());
        context.properties_mut().insert("test".into(), "ok".into());
        context.properties_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = AggregateMetricTelemetry::new("test".into());
        telemetry.stats_mut().add_data(&[9.0, 10.0, 11.0, 7.0, 13.0]);
        telemetry.properties_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.Metric",
            "2019-01-02T03:04:05.100Z",
        )
        .data(Base::Data(Data::MetricData(
            MetricDataBuilder::new(
                DataPointBuilder::new("test", 50)
                    .std_dev(2)
                    .min(7)
                    .max(13)
                    .count(5)
                    .kind(DataPointType::Aggregation)
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
        SystemTime::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 101));

        let mut context = TelemetryContext::new("instrumentation".into());
        context.tags_mut().insert("test".into(), "ok".into());
        context.tags_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = AggregateMetricTelemetry::new("test".into());
        telemetry.stats_mut().add_data(&[9.0, 10.0, 11.0, 7.0, 13.0]);
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.Metric",
            "2019-01-02T03:04:05.101Z",
        )
        .data(Base::Data(Data::MetricData(
            MetricDataBuilder::new(
                DataPointBuilder::new("test", 50)
                    .std_dev(2)
                    .min(7)
                    .max(13)
                    .count(5)
                    .kind(DataPointType::Aggregation)
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

    #[test]
    fn it_updates_stats() {
        let mut stats = Stats::default();
        stats.add_data(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);

        let mut telemetry = AggregateMetricTelemetry::new("stats".into());
        *telemetry.stats_mut() = stats;

        assert_eq!(telemetry.stats().value, 15.0);
    }

    #[test]
    fn it_updates_properties() {
        let mut properties = Properties::default();
        properties.insert("name".into(), "value".into());

        let mut telemetry = AggregateMetricTelemetry::new("props".into());
        *telemetry.properties_mut() = properties;

        assert_eq!(telemetry.properties().len(), 1);
    }

    #[test]
    fn it_updates_tags() {
        let mut tags = ContextTags::default();
        tags.insert("name".into(), "value".into());

        let mut telemetry = AggregateMetricTelemetry::new("props".into());
        *telemetry.tags_mut() = tags;

        assert_eq!(telemetry.tags().len(), 1);
    }
}
