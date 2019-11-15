use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Request represents completion of an external request to the application to do work and contains a summary of that request execution and the results.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestData {
    ver: i32,
    id: String,
    source: Option<String>,
    name: Option<String>,
    duration: String,
    response_code: String,
    success: bool,
    url: Option<String>,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
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
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl RequestDataBuilder {
    /// Creates a new [RequestDataBuilder](trait.RequestDataBuilder.html) instance with default values set by the schema.
    pub fn new(id: impl Into<String>, duration: impl Into<String>, response_code: impl Into<String>) -> Self {
        Self {
            ver: 2,
            id: id.into(),
            source: None,
            name: None,
            duration: duration.into(),
            response_code: response_code.into(),
            success: true,
            url: None,
            properties: None,
            measurements: None,
        }
    }

    /// Sets: Schema version
    pub fn ver(&mut self, ver: impl Into<i32>) -> &mut Self {
        self.ver = ver.into();
        self
    }

    /// Sets: Source of the request. Examples are the instrumentation key of the caller or the ip address of the caller.
    pub fn source(&mut self, source: impl Into<String>) -> &mut Self {
        self.source = Some(source.into());
        self
    }

    /// Sets: Name of the request. Represents code path taken to process request. Low cardinality value to allow better grouping of requests. For HTTP requests it represents the HTTP method and URL path template like 'GET /values/{id}'.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    /// Sets: Indication of successfull or unsuccessfull call.
    pub fn success(&mut self, success: impl Into<bool>) -> &mut Self {
        self.success = success.into();
        self
    }

    /// Sets: Request URL with all query string parameters.
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }

    /// Sets: Collection of custom properties.
    pub fn properties(&mut self, properties: impl Into<std::collections::BTreeMap<String, String>>) -> &mut Self {
        self.properties = Some(properties.into());
        self
    }

    /// Sets: Collection of custom measurements.
    pub fn measurements(&mut self, measurements: impl Into<std::collections::BTreeMap<String, f64>>) -> &mut Self {
        self.measurements = Some(measurements.into());
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
