use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Request represents completion of an external request to the application to do work and contains a summary of that request execution and the results.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestData {
    pub ver: i32,
    pub id: String,
    pub source: Option<String>,
    pub name: Option<String>,
    pub duration: String,
    pub response_code: String,
    pub success: bool,
    pub url: Option<String>,
    pub properties: Option<std::collections::BTreeMap<String, String>>,
    pub measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl Default for RequestData {
    fn default() -> Self {
        Self {
            ver: 2,
            id: String::default(),
            source: Option::default(),
            name: Option::default(),
            duration: String::default(),
            response_code: String::default(),
            success: true,
            url: Option::default(),
            properties: Option::default(),
            measurements: Option::default(),
        }
    }
}
