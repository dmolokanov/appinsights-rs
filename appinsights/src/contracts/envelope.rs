use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// System variables for a telemetry item.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Envelope {
    pub ver: Option<i32>,
    pub name: String,
    pub time: String,
    pub sample_rate: Option<f64>,
    pub seq: Option<String>,
    pub i_key: Option<String>,
    pub flags: Option<i64>,
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    pub data: Option<Base>,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            ver: Some(1),
            name: String::default(),
            time: String::default(),
            sample_rate: Some(100.0),
            seq: Option::default(),
            i_key: Option::default(),
            flags: Option::default(),
            tags: Option::default(),
            data: Option::default(),
        }
    }
}
