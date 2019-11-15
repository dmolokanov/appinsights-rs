use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Metric data single measurement.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPoint {
    ns: Option<String>,
    name: String,
    kind: Option<DataPointType>,
    value: f64,
    count: Option<i32>,
    min: Option<f64>,
    max: Option<f64>,
    std_dev: Option<f64>,
}

/// Creates: Metric data single measurement.
#[derive(Debug, Clone)]
pub struct DataPointBuilder {
    ns: Option<String>,
    name: String,
    kind: Option<DataPointType>,
    value: f64,
    count: Option<i32>,
    min: Option<f64>,
    max: Option<f64>,
    std_dev: Option<f64>,
}

impl DataPointBuilder {
    /// Creates a new [DataPointBuilder](trait.DataPointBuilder.html) instance with default values set by the schema.
    pub fn new(name: impl Into<String>, value: impl Into<f64>) -> Self {
        Self {
            ns: None,
            name: name.into(),
            kind: Some(DataPointType::Measurement),
            value: value.into(),
            count: None,
            min: None,
            max: None,
            std_dev: None,
        }
    }

    /// Sets: Namespace of the metric.
    pub fn ns(&mut self, ns: impl Into<String>) -> &mut Self {
        self.ns = Some(ns.into());
        self
    }

    /// Sets: Metric type. Single measurement or the aggregated value.
    pub fn kind(&mut self, kind: impl Into<DataPointType>) -> &mut Self {
        self.kind = Some(kind.into());
        self
    }

    /// Sets: Metric weight of the aggregated metric. Should not be set for a measurement.
    pub fn count(&mut self, count: impl Into<i32>) -> &mut Self {
        self.count = Some(count.into());
        self
    }

    /// Sets: Minimum value of the aggregated metric. Should not be set for a measurement.
    pub fn min(&mut self, min: impl Into<f64>) -> &mut Self {
        self.min = Some(min.into());
        self
    }

    /// Sets: Maximum value of the aggregated metric. Should not be set for a measurement.
    pub fn max(&mut self, max: impl Into<f64>) -> &mut Self {
        self.max = Some(max.into());
        self
    }

    /// Sets: Standard deviation of the aggregated metric. Should not be set for a measurement.
    pub fn std_dev(&mut self, std_dev: impl Into<f64>) -> &mut Self {
        self.std_dev = Some(std_dev.into());
        self
    }

    /// Creates a new [DataPoint](trait.DataPoint.html) instance with values from [DataPointBuilder](trait.DataPointBuilder.html).
    pub fn build(&self) -> DataPoint {
        DataPoint {
            ns: self.ns.clone(),
            name: self.name.clone(),
            kind: self.kind.clone(),
            value: self.value.clone(),
            count: self.count.clone(),
            min: self.min.clone(),
            max: self.max.clone(),
            std_dev: self.std_dev.clone(),
        }
    }
}
