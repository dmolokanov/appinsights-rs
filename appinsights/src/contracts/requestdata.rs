use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Request represents completion of an external request to the application to do work and contains a summary of that request execution and the results.
#[derive(Debug, Serialize)]
pub struct RequestData {
    ver: i32,
    id: String,
    source: Option<String>,
    name: Option<String>,
    duration: String,
    response_code: String,
    success: bool,
    url: Option<String>,
    properties: Option<std::collections::HashMap<String, String>>,
    measurements: Option<std::collections::HashMap<String, f64>>,
}

impl RequestData {
    /// Create a new [RequestData](trait.RequestData.html) instance with default values set by the schema.
    pub fn new(id: String, duration: String, response_code: String) -> Self {
        Self {
            ver: 2,
            id,
            source: None,
            name: None,
            duration,
            response_code,
            success: true,
            url: None,
            properties: None,
            measurements: None,
        }
    }

    /// Schema version
    pub fn with_ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Identifier of a request call instance. Used for correlation between request and other telemetry items.
    pub fn with_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// Source of the request. Examples are the instrumentation key of the caller or the ip address of the caller.
    pub fn with_source(&mut self, source: Option<String>) -> &mut Self {
        self.source = source;
        self
    }

    /// Name of the request. Represents code path taken to process request. Low cardinality value to allow better grouping of requests. For HTTP requests it represents the HTTP method and URL path template like 'GET /values/{id}'.
    pub fn with_name(&mut self, name: Option<String>) -> &mut Self {
        self.name = name;
        self
    }

    /// Request duration in format: DD.HH:MM:SS.MMMMMM. Must be less than 1000 days.
    pub fn with_duration(&mut self, duration: String) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Result of a request execution. HTTP status code for HTTP requests.
    pub fn with_response_code(&mut self, response_code: String) -> &mut Self {
        self.response_code = response_code;
        self
    }

    /// Indication of successfull or unsuccessfull call.
    pub fn with_success(&mut self, success: bool) -> &mut Self {
        self.success = success;
        self
    }

    /// Request URL with all query string parameters.
    pub fn with_url(&mut self, url: Option<String>) -> &mut Self {
        self.url = url;
        self
    }

    /// Collection of custom properties.
    pub fn with_properties(&mut self, properties: Option<std::collections::HashMap<String, String>>) -> &mut Self {
        self.properties = properties;
        self
    }

    /// Collection of custom measurements.
    pub fn with_measurements(&mut self, measurements: Option<std::collections::HashMap<String, f64>>) -> &mut Self {
        self.measurements = measurements;
        self
    }
}

impl TelemetryData for RequestData {
    /// Returns the base type when placed within an [Data](trait.Data.html) container.
    fn base_type(&self) -> String {
        String::from("RequestData")
    }
}
