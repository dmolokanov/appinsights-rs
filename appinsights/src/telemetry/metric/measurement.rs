use chrono::{DateTime, SecondsFormat, Utc};

use crate::context::TelemetryContext;
use crate::contracts::*;
use crate::telemetry::{ContextTags, Properties, Telemetry};
use crate::time;

/// Metric telemetry item that represents a single data point.
///
/// # Examples
/// ```rust, no_run
/// # use appinsights::TelemetryClient;
/// # let client = TelemetryClient::new("<instrumentation key>".to_string());
/// use appinsights::telemetry::{Telemetry, MetricTelemetry};
///
/// // create a telemetry item
/// let mut telemetry = MetricTelemetry::new("temp_sensor".to_string(), 55.0);
///
/// // assign custom properties and context tags
/// telemetry.properties_mut().insert("component".to_string(), "external_device".to_string());
/// telemetry.tags_mut().insert("os_version".to_string(), "linux x86_64".to_string());
///
/// // submit telemetry item to server
/// client.track(telemetry);
/// ```
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
    pub fn new(name: String, value: f64) -> Self {
        Self {
            name,
            value,
            timestamp: time::now(),
            properties: Properties::default(),
            tags: ContextTags::default(),
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
        Self {
            name: "Microsoft.ApplicationInsights.Metric".into(),
            time: telemetry.timestamp.to_rfc3339_opts(SecondsFormat::Millis, true),
            i_key: Some(context.i_key),
            tags: Some(ContextTags::combine(context.tags, telemetry.tags).into()),
            data: Some(Base::Data(Data::MetricData(MetricData {
                metrics: vec![DataPoint {
                    name: telemetry.name,
                    kind: Some(DataPointType::Measurement),
                    value: telemetry.value,
                    count: Some(1),
                    ..DataPoint::default()
                }],
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

        let mut telemetry = MetricTelemetry::new("test".into(), 123.0);
        telemetry.properties_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.Metric".into(),
            time: "2019-01-02T03:04:05.100Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some(BTreeMap::default()),
            data: Some(Base::Data(Data::MetricData(MetricData {
                metrics: vec![DataPoint {
                    name: "test".into(),
                    kind: Some(DataPointType::Measurement),
                    value: 123.0,
                    count: Some(1),
                    ..DataPoint::default()
                }],
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

        let mut telemetry = MetricTelemetry::new("test".into(), 123.0);
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
                metrics: vec![DataPoint {
                    name: "test".into(),
                    kind: Some(DataPointType::Measurement),
                    value: 123.0,
                    count: Some(1),
                    ..DataPoint::default()
                }],
                properties: Some(BTreeMap::default()),
                ..MetricData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }
}
