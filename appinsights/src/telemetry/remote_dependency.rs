use std::time::Duration as StdDuration;

use chrono::{DateTime, SecondsFormat, Utc};

use crate::context::TelemetryContext;
use crate::contracts::*;
use crate::telemetry::{ContextTags, Measurements, Properties, Telemetry};
use crate::time::{self, Duration};
use crate::uuid::Uuid;

/// Represents interactions of the monitored component with a remote component/service like SQL or an HTTP endpoint.
///
/// # Examples
/// ```rust, no_run
/// # use appinsights::TelemetryClient;
/// # let client = TelemetryClient::new("<instrumentation key>".to_string());
/// use appinsights::telemetry::{Telemetry, RemoteDependencyTelemetry};
/// use std::time::Duration;
///
/// // create a telemetry item
/// let mut telemetry = RemoteDependencyTelemetry::new(
///     "GET https://api.github.com/dmolokanov/appinsights-rs".to_string(),
///     "HTTP".into(),
///     Duration::from_secs(2),
///     "api.github.com".to_string(),
///     true,
/// );
///
/// // attach custom properties, measurements and context tags
/// telemetry.properties_mut().insert("component".to_string(), "data_processor".to_string());
/// telemetry.tags_mut().insert("os_version".to_string(), "linux x86_64".to_string());
/// telemetry.measurements_mut().insert("body_size".to_string(), 115.0);
///
/// // submit telemetry item to server
/// client.track(telemetry);
/// ```
pub struct RemoteDependencyTelemetry {
    /// Identifier of a dependency call instance.
    /// It is used for correlation with the request telemetry item corresponding to this dependency call.
    id: Option<Uuid>,

    /// Name of the command that initiated this dependency call. Low cardinality value.
    /// Examples are stored procedure name and URL path template.
    name: String,

    /// Duration of the remote call.
    duration: Duration,

    /// Result code of a dependency call.
    /// Examples are SQL error code and HTTP status code.
    result_code: Option<String>,

    /// Indication of successful or unsuccessful call.
    success: bool,

    /// Command initiated by this dependency call.
    /// Examples are SQL statement and HTTP URL's with all the query parameters.
    data: Option<String>,

    /// Dependency type name. Very low cardinality.
    /// Examples are SQL, Azure table and HTTP.
    dependency_type: String,

    /// Target site of a dependency call.
    /// Examples are server name, host address.
    target: String,

    /// The time stamp when this telemetry was measured.
    timestamp: DateTime<Utc>,

    /// Custom properties.
    properties: Properties,

    /// Telemetry context containing extra, optional tags.
    tags: ContextTags,

    /// Custom measurements.
    measurements: Measurements,
}

impl RemoteDependencyTelemetry {
    /// Creates a new telemetry item with specified name, dependency type, target site and success status.
    pub fn new(name: String, dependency_type: String, duration: StdDuration, target: String, success: bool) -> Self {
        Self {
            id: Option::default(),
            name,
            duration: duration.into(),
            result_code: Option::default(),
            success,
            data: Option::default(),
            dependency_type,
            target,
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

impl Telemetry for RemoteDependencyTelemetry {
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

impl From<(TelemetryContext, RemoteDependencyTelemetry)> for Envelope {
    fn from((context, telemetry): (TelemetryContext, RemoteDependencyTelemetry)) -> Self {
        Self {
            name: "Microsoft.ApplicationInsights.RemoteDependency".into(),
            time: telemetry.timestamp.to_rfc3339_opts(SecondsFormat::Millis, true),
            i_key: Some(context.i_key),
            tags: Some(ContextTags::combine(context.tags, telemetry.tags).into()),
            data: Some(Base::Data(Data::RemoteDependencyData(RemoteDependencyData {
                name: telemetry.name,
                id: telemetry.id.map(|id| id.to_hyphenated().to_string()),
                result_code: telemetry.result_code,
                duration: telemetry.duration.to_string(),
                success: Some(telemetry.success),
                data: telemetry.data,
                target: Some(telemetry.target),
                type_: Some(telemetry.dependency_type),
                properties: Some(Properties::combine(context.properties, telemetry.properties).into()),
                measurements: Some(telemetry.measurements.into()),
                ..RemoteDependencyData::default()
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

    #[test]
    fn it_overrides_properties_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 800));

        let mut context = TelemetryContext::with_i_key("instrumentation".into());
        context.properties_mut().insert("test".into(), "ok".into());
        context.properties_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = RemoteDependencyTelemetry::new(
            "GET https://example.com/main.html".into(),
            "HTTP".into(),
            StdDuration::from_secs(2),
            "example.com".into(),
            true,
        );
        telemetry.properties_mut().insert("no-write".into(), "ok".into());
        telemetry.measurements_mut().insert("latency".into(), 200.0);

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.RemoteDependency".into(),
            time: "2019-01-02T03:04:05.800Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some(BTreeMap::default()),
            data: Some(Base::Data(Data::RemoteDependencyData(RemoteDependencyData {
                name: "GET https://example.com/main.html".into(),
                duration: "0.00:00:02.0000000".into(),
                success: Some(true),
                target: Some("example.com".into()),
                type_: Some("HTTP".into()),
                properties: Some({
                    let mut properties = BTreeMap::default();
                    properties.insert("test".into(), "ok".into());
                    properties.insert("no-write".into(), "ok".into());
                    properties
                }),
                measurements: Some({
                    let mut measurements = BTreeMap::default();
                    measurements.insert("latency".into(), 200.0);
                    measurements
                }),
                ..RemoteDependencyData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }

    #[test]
    fn it_overrides_tags_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 700));

        let mut context = TelemetryContext::with_i_key("instrumentation".into());
        context.tags_mut().insert("test".into(), "ok".into());
        context.tags_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = RemoteDependencyTelemetry::new(
            "GET https://example.com/main.html".into(),
            "HTTP".into(),
            StdDuration::from_secs(2),
            "example.com".into(),
            true,
        );
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.RemoteDependency".into(),
            time: "2019-01-02T03:04:05.700Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some({
                let mut tags = BTreeMap::default();
                tags.insert("test".into(), "ok".into());
                tags.insert("no-write".into(), "ok".into());
                tags
            }),
            data: Some(Base::Data(Data::RemoteDependencyData(RemoteDependencyData {
                name: "GET https://example.com/main.html".into(),
                duration: "0.00:00:02.0000000".into(),
                success: Some(true),
                target: Some("example.com".into()),
                type_: Some("HTTP".into()),
                properties: Some(BTreeMap::default()),
                measurements: Some(BTreeMap::default()),
                ..RemoteDependencyData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }
}
