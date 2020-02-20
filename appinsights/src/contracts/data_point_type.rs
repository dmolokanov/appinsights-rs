use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Type of the metric data measurement.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DataPointType {
    Measurement,
    Aggregation,
}
