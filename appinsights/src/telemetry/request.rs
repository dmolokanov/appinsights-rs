use std::str::FromStr;
use std::time::Duration as StdDuration;

use chrono::{DateTime, SecondsFormat, Utc};
use http::{Method, StatusCode, Uri};

use crate::context::TelemetryContext;
use crate::contracts::*;
use crate::telemetry::{ContextTags, Measurements, Properties, Telemetry};
use crate::time::{self, Duration};
use crate::uuid::{self, Uuid};

/// Represents completion of an external request to the application and contains a summary of that
/// request execution and results. This struct is focused on HTTP requests.
///
/// # Examples
/// ```rust, no_run
/// # use appinsights::TelemetryClient;
/// # let client = TelemetryClient::new("<instrumentation key>".to_string());
/// use appinsights::telemetry::{Telemetry, RequestTelemetry};
/// use http::{Method, Uri};
/// use std::time::Duration;
///
/// // create a telemetry item
/// let mut telemetry = RequestTelemetry::new(
///     Method::GET,
///     "https://api.github.com/dmolokanov/appinsights-rs".parse::<Uri>().unwrap(),
///     Duration::from_millis(182),
///     "200"
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
pub struct RequestTelemetry {
    /// Identifier of a request call instance.
    /// It is used for correlation between request and other telemetry items.
    id: Uuid,

    /// Request name. For HTTP requests it represents the HTTP method and URL path template.
    name: String,

    /// URL of the request with all query string parameters.
    uri: Uri,

    /// Duration to serve the request.
    duration: Duration,

    /// Results of a request execution. HTTP status code for HTTP requests.
    response_code: String,

    /// The time stamp when this telemetry was measured.
    timestamp: DateTime<Utc>,

    /// Custom properties.
    properties: Properties,

    /// Telemetry context containing extra, optional tags.
    tags: ContextTags,

    /// Custom measurements.
    measurements: Measurements,
}

impl RequestTelemetry {
    /// Creates a new telemetry item for HTTP request.
    pub fn new(method: Method, uri: Uri, duration: StdDuration, response_code: impl Into<String>) -> Self {
        let mut authority = String::new();
        if let Some(host) = &uri.host() {
            authority.push_str(host);
        }
        if let Some(port) = &uri.port_u16() {
            authority.push_str(&format!(":{}", port))
        }

        let uri = Uri::builder()
            .scheme(uri.scheme_str().unwrap_or_default())
            .authority(authority.as_str())
            .path_and_query(uri.path())
            .build()
            .unwrap_or(uri);

        let name = format!("{} {}", method, uri);

        let mut tags = ContextTags::default();
        tags.operation_mut().set_name(name.clone());

        Self {
            id: uuid::new(),
            name,
            uri,
            duration: duration.into(),
            response_code: response_code.into(),
            timestamp: time::now(),
            properties: Properties::default(),
            tags,
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

    /// Returns an indication of successful or unsuccessful call.
    pub fn is_success(&self) -> bool {
        if let Ok(response_code) = StatusCode::from_str(&self.response_code) {
            response_code < StatusCode::BAD_REQUEST || response_code == StatusCode::UNAUTHORIZED
        } else {
            true
        }
    }
}

impl Telemetry for RequestTelemetry {
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

impl From<(TelemetryContext, RequestTelemetry)> for Envelope {
    fn from((context, telemetry): (TelemetryContext, RequestTelemetry)) -> Self {
        let success = telemetry.is_success();
        Self {
            name: "Microsoft.ApplicationInsights.Request".into(),
            time: telemetry.timestamp.to_rfc3339_opts(SecondsFormat::Millis, true),
            i_key: Some(context.i_key),
            tags: Some(ContextTags::combine(context.tags, telemetry.tags).into()),
            data: Some(Base::Data(Data::RequestData(RequestData {
                id: telemetry.id.to_hyphenated().to_string(),
                name: Some(telemetry.name),
                duration: telemetry.duration.to_string(),
                response_code: telemetry.response_code,
                success,
                url: Some(telemetry.uri.to_string()),
                properties: Some(Properties::combine(context.properties, telemetry.properties).into()),
                measurements: Some(telemetry.measurements.into()),
                ..RequestData::default()
            }))),
            ..Envelope::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::str::FromStr;

    use chrono::TimeZone;

    use super::*;
    use crate::uuid::{self, Uuid};

    #[test]
    fn it_overrides_properties_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 800));
        uuid::set(Uuid::from_str("910b414a-f368-4b3a-aff6-326632aac566").unwrap());

        let mut context =
            TelemetryContext::new("instrumentation".into(), ContextTags::default(), Properties::default());
        context.properties_mut().insert("test".into(), "ok".into());
        context.properties_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = RequestTelemetry::new(
            Method::GET,
            "https://example.com/main.html".parse().unwrap(),
            StdDuration::from_secs(2),
            "200",
        );
        telemetry.properties_mut().insert("no-write".into(), "ok".into());
        telemetry.measurements_mut().insert("latency".into(), 200.0);

        let envelop = Envelope::from((context, telemetry));

        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.Request".into(),
            time: "2019-01-02T03:04:05.800Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some({
                let mut tags = BTreeMap::default();
                tags.insert("ai.operation.name".into(), "GET https://example.com/main.html".into());
                tags
            }),
            data: Some(Base::Data(Data::RequestData(RequestData {
                id: "910b414a-f368-4b3a-aff6-326632aac566".into(),
                name: Some("GET https://example.com/main.html".into()),
                duration: "0.00:00:02.0000000".into(),
                response_code: "200".into(),
                success: true,
                url: Some("https://example.com/main.html".into()),
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
                ..RequestData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }

    #[test]
    fn it_overrides_tags_from_context() {
        time::set(Utc.ymd(2019, 1, 2).and_hms_milli(3, 4, 5, 700));
        uuid::set(Uuid::from_str("910b414a-f368-4b3a-aff6-326632aac566").unwrap());

        let mut context =
            TelemetryContext::new("instrumentation".into(), ContextTags::default(), Properties::default());
        context.tags_mut().insert("test".into(), "ok".into());
        context.tags_mut().insert("no-write".into(), "fail".into());

        let mut telemetry = RequestTelemetry::new(
            Method::GET,
            "https://example.com/main.html".parse().unwrap(),
            StdDuration::from_secs(2),
            "200",
        );
        telemetry.tags_mut().insert("no-write".into(), "ok".into());

        let envelop = Envelope::from((context, telemetry));
        let expected = Envelope {
            name: "Microsoft.ApplicationInsights.Request".into(),
            time: "2019-01-02T03:04:05.700Z".into(),
            i_key: Some("instrumentation".into()),
            tags: Some({
                let mut tags = BTreeMap::default();
                tags.insert("ai.operation.name".into(), "GET https://example.com/main.html".into());
                tags.insert("test".into(), "ok".into());
                tags.insert("no-write".into(), "ok".into());
                tags
            }),
            data: Some(Base::Data(Data::RequestData(RequestData {
                id: "910b414a-f368-4b3a-aff6-326632aac566".into(),
                name: Some("GET https://example.com/main.html".into()),
                duration: "0.00:00:02.0000000".into(),
                response_code: "200".into(),
                success: true,
                url: Some("https://example.com/main.html".into()),
                properties: Some(BTreeMap::default()),
                measurements: Some(BTreeMap::default()),
                ..RequestData::default()
            }))),
            ..Envelope::default()
        };

        assert_eq!(envelop, expected)
    }
}
