use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Type of the metric data measurement.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DataPointType {
    Measurement,
    Aggregation,
}
