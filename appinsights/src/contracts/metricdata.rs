use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of the Metric item is a list of measurements (single data points) and/or aggregations.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricData {
    pub ver: i32,
    pub metrics: DataPoint,
    pub properties: Option<std::collections::BTreeMap<String, String>>,
}

impl Default for MetricData {
    fn default() -> Self {
        Self {
            ver: 2,
            metrics: DataPoint::default(),
            properties: Option::default(),
        }
    }
}
