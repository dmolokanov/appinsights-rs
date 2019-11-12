use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Request represents completion of an external request to the application to do work and contains a summary of that request execution and the results.
#[derive(Debug, Clone, Serialize)]
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

/// Creates: An instance of Request represents completion of an external request to the application to do work and contains a summary of that request execution and the results.
#[derive(Debug, Clone)]
pub struct RequestDataBuilder {
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

impl RequestDataBuilder {
    /// Creates a new [RequestDataBuilder](trait.RequestDataBuilder.html) instance with default values set by the schema.
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

    /// Sets: Schema version
    pub fn ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Sets: Source of the request. Examples are the instrumentation key of the caller or the ip address of the caller.
    pub fn source(&mut self, source: String) -> &mut Self {
        self.source = Some(source);
        self
    }

    /// Sets: Name of the request. Represents code path taken to process request. Low cardinality value to allow better grouping of requests. For HTTP requests it represents the HTTP method and URL path template like 'GET /values/{id}'.
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    /// Sets: Indication of successfull or unsuccessfull call.
    pub fn success(&mut self, success: bool) -> &mut Self {
        self.success = success;
        self
    }

    /// Sets: Request URL with all query string parameters.
    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    /// Sets: Collection of custom properties.
    pub fn properties(&mut self, properties: std::collections::HashMap<String, String>) -> &mut Self {
        self.properties = Some(properties);
        self
    }

    /// Sets: Collection of custom measurements.
    pub fn measurements(&mut self, measurements: std::collections::HashMap<String, f64>) -> &mut Self {
        self.measurements = Some(measurements);
        self
    }

    /// Creates a new [RequestData](trait.RequestData.html) instance with values from [RequestDataBuilder](trait.RequestDataBuilder.html).
    pub fn build(&self) -> RequestData {
        RequestData {
            ver: self.ver.clone(),
            id: self.id.clone(),
            source: self.source.clone(),
            name: self.name.clone(),
            duration: self.duration.clone(),
            response_code: self.response_code.clone(),
            success: self.success.clone(),
            url: self.url.clone(),
            properties: self.properties.clone(),
            measurements: self.measurements.clone(),
        }
    }
}

impl TelemetryData for RequestData {
    /// Returns the base type when placed within an [Data](trait.Data.html) container.
    fn base_type(&self) -> String {
        String::from("RequestData")
    }
}
