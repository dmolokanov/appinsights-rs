use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Exception represents a handled or unhandled exception that occurred during execution of the monitored application.
#[derive(Debug, Serialize)]
pub struct ExceptionData {
    ver: i32,
    exceptions: ExceptionDetails,
    severity_level: Option<SeverityLevel>,
    problem_id: Option<String>,
    properties: Option<std::collections::HashMap<String, String>>,
    measurements: Option<std::collections::HashMap<String, f64>>,
}

impl ExceptionData {
    /// Create a new [ExceptionData](trait.ExceptionData.html) instance with default values set by the schema.
    pub fn new(exceptions: ExceptionDetails) -> Self {
        Self {
            ver: 2,
            exceptions,
            severity_level: None,
            problem_id: None,
            properties: None,
            measurements: None,
        }
    }

    /// Schema version
    pub fn with_ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Exception chain - list of inner exceptions.
    pub fn with_exceptions(&mut self, exceptions: ExceptionDetails) -> &mut Self {
        self.exceptions = exceptions;
        self
    }

    /// Severity level. Mostly used to indicate exception severity level when it is reported by logging library.
    pub fn with_severity_level(&mut self, severity_level: Option<SeverityLevel>) -> &mut Self {
        self.severity_level = severity_level;
        self
    }

    /// Identifier of where the exception was thrown in code. Used for exceptions grouping. Typically a combination of exception type and a function from the call stack.
    pub fn with_problem_id(&mut self, problem_id: Option<String>) -> &mut Self {
        self.problem_id = problem_id;
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

impl TelemetryData for ExceptionData {
    /// Returns the base type when placed within an [Data](trait.Data.html) container.
    fn base_type(&self) -> String {
        String::from("ExceptionData")
    }
}
