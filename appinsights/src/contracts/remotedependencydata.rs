use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Remote Dependency represents an interaction of the monitored component with a remote component/service like SQL or an HTTP endpoint.
#[derive(Debug, Serialize)]
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
    properties: Option<std::collections::HashMap<String, String>>,
    measurements: Option<std::collections::HashMap<String, f64>>,
}

impl RemoteDependencyData {
    /// Create a new [RemoteDependencyData](trait.RemoteDependencyData.html) instance with default values set by the schema.
    pub fn new(name: String, duration: String) -> Self {
        Self {
            ver: 2,
            name,
            id: None,
            result_code: None,
            duration,
            success: Some(true),
            data: None,
            target: None,
            type_: None,
            properties: None,
            measurements: None,
        }
    }

    /// Schema version
    pub fn with_ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Name of the command initiated with this dependency call. Low cardinality value. Examples are stored procedure name and URL path template.
    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Identifier of a dependency call instance. Used for correlation with the request telemetry item corresponding to this dependency call.
    pub fn with_id(&mut self, id: Option<String>) -> &mut Self {
        self.id = id;
        self
    }

    /// Result code of a dependency call. Examples are SQL error code and HTTP status code.
    pub fn with_result_code(&mut self, result_code: Option<String>) -> &mut Self {
        self.result_code = result_code;
        self
    }

    /// Request duration in format: DD.HH:MM:SS.MMMMMM. Must be less than 1000 days.
    pub fn with_duration(&mut self, duration: String) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Indication of successfull or unsuccessfull call.
    pub fn with_success(&mut self, success: Option<bool>) -> &mut Self {
        self.success = success;
        self
    }

    /// Command initiated by this dependency call. Examples are SQL statement and HTTP URL's with all query parameters.
    pub fn with_data(&mut self, data: Option<String>) -> &mut Self {
        self.data = data;
        self
    }

    /// Target site of a dependency call. Examples are server name, host address.
    pub fn with_target(&mut self, target: Option<String>) -> &mut Self {
        self.target = target;
        self
    }

    /// Dependency type name. Very low cardinality value for logical grouping of dependencies and interpretation of other fields like commandName and resultCode. Examples are SQL, Azure table, and HTTP.
    pub fn with_type_(&mut self, type_: Option<String>) -> &mut Self {
        self.type_ = type_;
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
