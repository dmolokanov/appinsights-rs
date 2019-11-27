use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of Remote Dependency represents an interaction of the monitored component with a remote component/service like SQL or an HTTP endpoint.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteDependencyData {
    pub ver: i32,
    pub name: String,
    pub id: Option<String>,
    pub result_code: Option<String>,
    pub duration: String,
    pub success: Option<bool>,
    pub data: Option<String>,
    pub target: Option<String>,
    pub type_: Option<String>,
    pub properties: Option<std::collections::BTreeMap<String, String>>,
    pub measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl Default for RemoteDependencyData {
    fn default() -> Self {
        Self {
            ver: 2,
            name: String::default(),
            id: Option::default(),
            result_code: Option::default(),
            duration: String::default(),
            success: Some(true),
            data: Option::default(),
            target: Option::default(),
            type_: Option::default(),
            properties: Option::default(),
            measurements: Option::default(),
        }
    }
}
