use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Exception represents a handled or unhandled exception that occurred during execution of the monitored application.
#[derive(Debug, Clone, Serialize)]
pub struct ExceptionData {
    ver: i32,
    exceptions: ExceptionDetails,
    severity_level: Option<SeverityLevel>,
    problem_id: Option<String>,
    properties: Option<std::collections::HashMap<String, String>>,
    measurements: Option<std::collections::HashMap<String, f64>>,
}

/// Creates: An instance of Exception represents a handled or unhandled exception that occurred during execution of the monitored application.
#[derive(Debug, Clone)]
pub struct ExceptionDataBuilder {
    ver: i32,
    exceptions: ExceptionDetails,
    severity_level: Option<SeverityLevel>,
    problem_id: Option<String>,
    properties: Option<std::collections::HashMap<String, String>>,
    measurements: Option<std::collections::HashMap<String, f64>>,
}

impl ExceptionDataBuilder {
    /// Creates a new [ExceptionDataBuilder](trait.ExceptionDataBuilder.html) instance with default values set by the schema.
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

    /// Sets: Schema version
    pub fn ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Sets: Severity level. Mostly used to indicate exception severity level when it is reported by logging library.
    pub fn severity_level(&mut self, severity_level: SeverityLevel) -> &mut Self {
        self.severity_level = Some(severity_level);
        self
    }

    /// Sets: Identifier of where the exception was thrown in code. Used for exceptions grouping. Typically a combination of exception type and a function from the call stack.
    pub fn problem_id(&mut self, problem_id: String) -> &mut Self {
        self.problem_id = Some(problem_id);
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

    /// Creates a new [ExceptionData](trait.ExceptionData.html) instance with values from [ExceptionDataBuilder](trait.ExceptionDataBuilder.html).
    pub fn build(&self) -> ExceptionData {
        ExceptionData {
            ver: self.ver.clone(),
            exceptions: self.exceptions.clone(),
            severity_level: self.severity_level.clone(),
            problem_id: self.problem_id.clone(),
            properties: self.properties.clone(),
            measurements: self.measurements.clone(),
        }
    }
}
