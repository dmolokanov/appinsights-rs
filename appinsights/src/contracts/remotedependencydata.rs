use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Remote Dependency represents an interaction of the monitored component with a remote component/service like SQL or an HTTP endpoint.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteDependencyData {
    ver: i32,
    name: String,
    id: Option<String>,
    result_code: Option<String>,
    duration: String,
    success: Option<bool>,
    data: Option<String>,
    target: Option<String>,
    type_: Option<String>,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

/// Creates: An instance of Remote Dependency represents an interaction of the monitored component with a remote component/service like SQL or an HTTP endpoint.
#[derive(Debug, Clone)]
pub struct RemoteDependencyDataBuilder {
    ver: i32,
    name: String,
    id: Option<String>,
    result_code: Option<String>,
    duration: String,
    success: Option<bool>,
    data: Option<String>,
    target: Option<String>,
    type_: Option<String>,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl RemoteDependencyDataBuilder {
    /// Creates a new [RemoteDependencyDataBuilder](trait.RemoteDependencyDataBuilder.html) instance with default values set by the schema.
    pub fn new(name: impl Into<String>, duration: impl Into<String>) -> Self {
        Self {
            ver: 2,
            name: name.into(),
            id: None,
            result_code: None,
            duration: duration.into(),
            success: Some(true),
            data: None,
            target: None,
            type_: None,
            properties: None,
            measurements: None,
        }
    }

    /// Sets: Schema version
    pub fn ver(&mut self, ver: impl Into<i32>) -> &mut Self {
        self.ver = ver.into();
        self
    }

    /// Sets: Identifier of a dependency call instance. Used for correlation with the request telemetry item corresponding to this dependency call.
    pub fn id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id = Some(id.into());
        self
    }

    /// Sets: Result code of a dependency call. Examples are SQL error code and HTTP status code.
    pub fn result_code(&mut self, result_code: impl Into<String>) -> &mut Self {
        self.result_code = Some(result_code.into());
        self
    }

    /// Sets: Indication of successfull or unsuccessfull call.
    pub fn success(&mut self, success: impl Into<bool>) -> &mut Self {
        self.success = Some(success.into());
        self
    }

    /// Sets: Command initiated by this dependency call. Examples are SQL statement and HTTP URL's with all query parameters.
    pub fn data(&mut self, data: impl Into<String>) -> &mut Self {
        self.data = Some(data.into());
        self
    }

    /// Sets: Target site of a dependency call. Examples are server name, host address.
    pub fn target(&mut self, target: impl Into<String>) -> &mut Self {
        self.target = Some(target.into());
        self
    }

    /// Sets: Dependency type name. Very low cardinality value for logical grouping of dependencies and interpretation of other fields like commandName and resultCode. Examples are SQL, Azure table, and HTTP.
    pub fn type_(&mut self, type_: impl Into<String>) -> &mut Self {
        self.type_ = Some(type_.into());
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

    /// Creates a new [RemoteDependencyData](trait.RemoteDependencyData.html) instance with values from [RemoteDependencyDataBuilder](trait.RemoteDependencyDataBuilder.html).
    pub fn build(&self) -> RemoteDependencyData {
        RemoteDependencyData {
            ver: self.ver.clone(),
            name: self.name.clone(),
            id: self.id.clone(),
            result_code: self.result_code.clone(),
            duration: self.duration.clone(),
            success: self.success.clone(),
            data: self.data.clone(),
            target: self.target.clone(),
            type_: self.type_.clone(),
            properties: self.properties.clone(),
            measurements: self.measurements.clone(),
        }
    }
}
