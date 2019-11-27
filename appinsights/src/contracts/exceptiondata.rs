use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Exception represents a handled or unhandled exception that occurred during execution of the monitored application.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionData {
    pub ver: i32,
    pub exceptions: ExceptionDetails,
    pub severity_level: Option<SeverityLevel>,
    pub problem_id: Option<String>,
    pub properties: Option<std::collections::BTreeMap<String, String>>,
    pub measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl Default for ExceptionData {
    fn default() -> Self {
        Self {
            ver: 2,
            exceptions: ExceptionDetails::default(),
            severity_level: Option::default(),
            problem_id: Option::default(),
            properties: Option::default(),
            measurements: Option::default(),
        }
    }
}
