use std::time::Duration as StdDuration;

use chrono::{DateTime, SecondsFormat, Utc};

use crate::context::TelemetryContext;
use crate::contracts::*;
use crate::telemetry::{ContextTags, Measurements, Properties, Telemetry};
use crate::time::{self, Duration};
use crate::uuid::Uuid;

/// Represents interactions of the monitored component with a remote component/service like SQL or an HTTP endpoint.
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
            id: Default::default(),
            name,
            duration: duration.into(),
            result_code: Default::default(),
            success,
            data: Default::default(),
            dependency_type,
            target,
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
        let data = Data::RemoteDependencyData({
            let mut builder = RemoteDependencyDataBuilder::new(telemetry.name, telemetry.duration.to_string());
            builder
                .type_(telemetry.dependency_type)
                .target(telemetry.target)
                .success(telemetry.success)
                .properties(Properties::combine(context.properties, telemetry.properties))
                .measurements(telemetry.measurements);

            if let Some(id) = telemetry.id {
                builder.id(id.to_hyphenated().to_string());
            }

            if let Some(result_code) = telemetry.result_code {
                builder.result_code(result_code);
            }

            if let Some(data) = telemetry.data {
                builder.data(data);
            }

            builder.build()
        });

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

    #[test]
    fn it_overrides_properties_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 800));

        let mut context = TelemetryContext::new("instrumentation".into());
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

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.RemoteDependency",
            "2019-01-02T03:04:05.800Z",
        )
        .data(Base::Data(Data::RemoteDependencyData(
            RemoteDependencyDataBuilder::new("GET https://example.com/main.html", "0.00:00:02.0000000")
                .type_("HTTP")
                .target("example.com")
                .success(true)
                .properties({
                    let mut properties = BTreeMap::default();
                    properties.insert("test".into(), "ok".into());
                    properties.insert("no-write".into(), "ok".into());
                    properties
                })
                .measurements({
                    let mut measurement = Measurements::default();
                    measurement.insert("latency".into(), 200.0);
                    measurement
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
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 700));

        let mut context = TelemetryContext::new("instrumentation".into());
        context.tags_mut().insert("test".into(), "ok".into());
        context.tags_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = RemoteDependencyTelemetry::new(
            "GET https://example.com/main.html".into(),
            "HTTP".into(),
            StdDuration::from_secs(2),
            "example.com".into(),
            true,
        );
        telemetry.measurements_mut().insert("latency".into(), 200.0);
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));

        let expected = EnvelopeBuilder::new(
            "Microsoft.ApplicationInsights.instrumentation.RemoteDependency",
            "2019-01-02T03:04:05.700Z",
        )
        .data(Base::Data(Data::RemoteDependencyData(
            RemoteDependencyDataBuilder::new("GET https://example.com/main.html", "0.00:00:02.0000000")
                .type_("HTTP")
                .target("example.com")
                .success(true)
                .properties(Properties::default())
                .measurements({
                    let mut measurement = Measurements::default();
                    measurement.insert("latency".into(), 200.0);
                    measurement
                })
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
