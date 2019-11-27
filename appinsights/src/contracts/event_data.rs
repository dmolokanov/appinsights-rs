use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Instances of Event represent structured event records that can be grouped and searched by their properties. Event data item also creates a metric of event count by name.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventData {
    pub ver: i32,
    pub name: String,
    pub properties: Option<std::collections::BTreeMap<String, String>>,
    pub measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl Default for EventData {
    fn default() -> Self {
        Self {
            ver: 2,
            name: String::default(),
            properties: Option::default(),
            measurements: Option::default(),
        }
    }
}
