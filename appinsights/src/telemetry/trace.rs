use chrono::{DateTime, SecondsFormat, Utc};

use crate::context::TelemetryContext;
use crate::contracts::{SeverityLevel as ContractsSeverityLevel, *};
use crate::telemetry::{ContextTags, Measurements, Properties, Telemetry};
use crate::time;

/// Represents printf-like trace statements that can be text searched. A trace telemetry items have
/// a message and an associated [`SeverityLevel`](enum.SeverityLevel.html).
///
/// # Examples
/// ```rust, no_run
/// # use appinsights::TelemetryClient;
/// # let client = TelemetryClient::new("<instrumentation key>".to_string());
/// use appinsights::telemetry::{TraceTelemetry, SeverityLevel, Telemetry};
///
/// // create a telemetry item
/// let mut telemetry = TraceTelemetry::new("Starting data processing".to_string(), SeverityLevel::Information);
///
/// // attach custom properties, measurements and context tags
/// telemetry.properties_mut().insert("component".to_string(), "data_processor".to_string());
/// telemetry.tags_mut().insert("os_version".to_string(), "linux x86_64".to_string());
/// telemetry.measurements_mut().insert("records_count".to_string(), 115.0);
///
/// // submit telemetry item to server
/// client.track(telemetry);
/// ```
pub struct TraceTelemetry {
    /// A trace message.
    message: String,

    /// Severity level.
    severity: SeverityLevel,

    /// The time stamp when this telemetry was measured.
    timestamp: DateTime<Utc>,

    /// Custom properties.
    properties: Properties,

    /// Telemetry context containing extra, optional tags.
    tags: ContextTags,

    /// Custom measurements.
    measurements: Measurements,
}

impl TraceTelemetry {
    /// Creates an event telemetry item with specified name.
    pub fn new(message: String, severity: SeverityLevel) -> Self {
        Self {
            message,
            severity,
            timestamp: time::now(),
            properties: Properties::default(),
            tags: ContextTags::default(),
            measurements: Measurements::default(),
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

impl Telemetry for TraceTelemetry {
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

impl From<(TelemetryContext, TraceTelemetry)> for Envelope {
    fn from((context, telemetry): (TelemetryContext, TraceTelemetry)) -> Self {
        Envelope {
            name: "Microsoft.ApplicationInsights.Message".into(),
            time: telemetry.timestamp.to_rfc3339_opts(SecondsFormat::Millis, true),
            i_key: Some(context.i_key),
            tags: Some(ContextTags::combine(context.tags, telemetry.tags).into()),
            data: Some(Base::Data(Data::MessageData(MessageData {
                message: telemetry.message,
                severity_level: Some(telemetry.severity.into()),
                properties: Some(Properties::combine(context.properties, telemetry.properties).into()),
                measurements: Some(telemetry.measurements.into()),
                ..MessageData::default()
            }))),
            ..Envelope::default()
        }
    }
}

/// Defines the level of severity for the event.
pub enum SeverityLevel {
    /// Verbose severity level.
    Verbose,

    /// Information severity level.
    Information,

    /// Warning severity level.
    Warning,

    /// Error severity level.
    Error,

    /// Critical severity level.
    Critical,
}

impl From<SeverityLevel> for ContractsSeverityLevel {
    fn from(severity: SeverityLevel) -> Self {
        match severity {
            SeverityLevel::Verbose => ContractsSeverityLevel::Verbose,
            SeverityLevel::Information => ContractsSeverityLevel::Information,
            SeverityLevel::Warning => ContractsSeverityLevel::Warning,
            SeverityLevel::Error => ContractsSeverityLevel::Error,
            SeverityLevel::Critical => ContractsSeverityLevel::Critical,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::TimeZone;

    use super::*;

    #[test]
    fn it_overrides_properties_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 800));

        let mut context = TelemetryContext::new("instrumentation".into());
        context.properties_mut().insert("test".into(), "ok".into());
        context.properties_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = TraceTelemetry::new("message".into(), SeverityLevel::Information);
        telemetry.properties_mut().insert("no-write".into(), "ok".into());
        telemetry.measurements_mut().insert("value".into(), 5.0);

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.Message".into(),
            time: "2019-01-02T03:04:05.800Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some(BTreeMap::default()),
            data: Some(Base::Data(Data::MessageData(MessageData {
                message: "message".into(),
                severity_level: Some(crate::contracts::SeverityLevel::Information),
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
                ..MessageData::default()
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

        let mut telemetry = TraceTelemetry::new("message".into(), SeverityLevel::Information);
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.Message".into(),
            time: "2019-01-02T03:04:05.700Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some({
                let mut tags = BTreeMap::default();
                tags.insert("test".into(), "ok".into());
                tags.insert("no-write".into(), "ok".into());
                tags
            }),
            data: Some(Base::Data(Data::MessageData(MessageData {
                message: "message".into(),
                severity_level: Some(crate::contracts::SeverityLevel::Information),
                properties: Some(BTreeMap::default()),
                measurements: Some(BTreeMap::default()),
                ..MessageData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }
}
