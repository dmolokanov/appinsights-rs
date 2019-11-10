use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Type of the metric data measurement.
#[derive(Debug, Serialize)]
pub enum DataPointType {
    Measurement,
    Aggregation,
}
