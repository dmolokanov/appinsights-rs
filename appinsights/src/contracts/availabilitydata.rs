use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Instances of AvailabilityData represent the result of executing an availability test.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityData {
    pub ver: i32,
    pub id: String,
    pub name: String,
    pub duration: String,
    pub success: bool,
    pub run_location: Option<String>,
    pub message: Option<String>,
    pub properties: Option<std::collections::BTreeMap<String, String>>,
    pub measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl Default for AvailabilityData {
    fn default() -> Self {
        Self {
            ver: 2,
            id: String::default(),
            name: String::default(),
            duration: String::default(),
            success: bool::default(),
            run_location: Option::default(),
            message: Option::default(),
            properties: Option::default(),
            measurements: Option::default(),
        }
    }
}
