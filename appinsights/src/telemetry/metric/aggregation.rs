use chrono::{DateTime, SecondsFormat, Utc};

use crate::context::TelemetryContext;
use crate::contracts::*;
use crate::telemetry::{ContextTags, Properties, Stats, Telemetry};
use crate::time;

/// Aggregated metric telemetry item that represents an aggregation of data points over time.
/// There values can be calculated by the caller or with [add_data](struct.Stats.html#method.add_data)
/// or [add_sampled_data](struct.Stats.html#method.add_sampled_data) method.
///
/// # Examples
/// ```rust, no_run
/// # use appinsights::TelemetryClient;
/// # let client = TelemetryClient::new("<instrumentation key>".to_string());
/// use appinsights::telemetry::{Telemetry, AggregateMetricTelemetry};
///
/// // create a telemetry item
/// let mut telemetry = AggregateMetricTelemetry::new("temp_sensor".into());
/// telemetry.stats_mut().add_data(&[50.0, 53.1, 56.4]);
///
/// // assign custom properties and context tags
/// telemetry.properties_mut().insert("component".to_string(), "external_device".to_string());
/// telemetry.tags_mut().insert("os_version".to_string(), "linux x86_64".to_string());
///
/// // submit telemetry item to server
/// client.track(telemetry);
/// ```
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
    pub fn new(name: String) -> Self {
        Self {
            name,
            stats: Stats::default(),
            timestamp: time::now(),
            properties: Properties::default(),
            tags: ContextTags::default(),
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
        Self {
            name: "Microsoft.ApplicationInsights.Metric".into(),
            time: telemetry.timestamp.to_rfc3339_opts(SecondsFormat::Millis, true),
            i_key: Some(context.i_key),
            tags: Some(ContextTags::combine(context.tags, telemetry.tags).into()),
            data: Some(Base::Data(Data::MetricData(MetricData {
                metrics: DataPoint {
                    name: telemetry.name,
                    kind: Some(DataPointType::Aggregation),
                    value: telemetry.stats.value,
                    count: Some(telemetry.stats.count),
                    min: Some(telemetry.stats.min),
                    max: Some(telemetry.stats.max),
                    std_dev: Some(telemetry.stats.std_dev),
                    ..DataPoint::default()
                },
                properties: Some(Properties::combine(context.properties, telemetry.properties).into()),
                ..MetricData::default()
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
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 100));

        let mut context =
            TelemetryContext::new("instrumentation".into(), ContextTags::default(), Properties::default());
        context.properties_mut().insert("test".into(), "ok".into());
        context.properties_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = AggregateMetricTelemetry::new("test".into());
        telemetry.stats_mut().add_data(&[9.0, 10.0, 11.0, 7.0, 13.0]);
        telemetry.properties_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.Metric".into(),
            time: "2019-01-02T03:04:05.100Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some(BTreeMap::default()),
            data: Some(Base::Data(Data::MetricData(MetricData {
                metrics: DataPoint {
                    name: "test".into(),
                    kind: Some(DataPointType::Aggregation),
                    value: 50.0,
                    count: Some(5),
                    min: Some(7.0),
                    max: Some(13.0),
                    std_dev: Some(2.0),
                    ..DataPoint::default()
                },
                properties: Some({
                    let mut properties = BTreeMap::default();
                    properties.insert("test".into(), "ok".into());
                    properties.insert("no-write".into(), "ok".into());
                    properties
                }),
                ..MetricData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }

    #[test]
    fn it_overrides_tags_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 101));

        let mut context =
            TelemetryContext::new("instrumentation".into(), ContextTags::default(), Properties::default());
        context.tags_mut().insert("test".into(), "ok".into());
        context.tags_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = AggregateMetricTelemetry::new("test".into());
        telemetry.stats_mut().add_data(&[9.0, 10.0, 11.0, 7.0, 13.0]);
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.Metric".into(),
            time: "2019-01-02T03:04:05.101Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some({
                let mut tags = BTreeMap::default();
                tags.insert("test".into(), "ok".into());
                tags.insert("no-write".into(), "ok".into());
                tags
            }),
            data: Some(Base::Data(Data::MetricData(MetricData {
                metrics: DataPoint {
                    name: "test".into(),
                    kind: Some(DataPointType::Aggregation),
                    value: 50.0,
                    count: Some(5),
                    min: Some(7.0),
                    max: Some(13.0),
                    std_dev: Some(2.0),
                    ..DataPoint::default()
                },
                properties: Some(BTreeMap::default()),
                ..MetricData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }

    #[test]
    fn it_updates_stats() {
        let mut stats = Stats::default();
        stats.add_data(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);

        let mut telemetry = AggregateMetricTelemetry::new("stats".into());
        *telemetry.stats_mut() = stats;

        assert!((telemetry.stats().value - 15.0).abs() < std::f64::EPSILON);
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
