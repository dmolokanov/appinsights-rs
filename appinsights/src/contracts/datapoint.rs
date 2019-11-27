use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Metric data single measurement.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPoint {
    pub ns: Option<String>,
    pub name: String,
    pub kind: Option<DataPointType>,
    pub value: f64,
    pub count: Option<i32>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub std_dev: Option<f64>,
}

impl Default for DataPoint {
    fn default() -> Self {
        Self {
            ns: None,
            name: String::default(),
            kind: Some(DataPointType::Measurement),
            value: f64::default(),
            count: None,
            min: None,
            max: None,
            std_dev: None,
        }
    }
}
