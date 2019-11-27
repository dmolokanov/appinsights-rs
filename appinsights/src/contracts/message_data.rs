use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Instances of Message represent printf-like trace statements that are text-searched. Log4Net, NLog and other text-based log file entries are translated into intances of this type. The message does not have measurements.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageData {
    pub ver: i32,
    pub message: String,
    pub severity_level: Option<SeverityLevel>,
    pub properties: Option<std::collections::BTreeMap<String, String>>,
    pub measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl Default for MessageData {
    fn default() -> Self {
        Self {
            ver: 2,
            message: String::default(),
            severity_level: Option::default(),
            properties: Option::default(),
            measurements: Option::default(),
        }
    }
}
