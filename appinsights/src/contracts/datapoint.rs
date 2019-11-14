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
    pub fn new(name: String, value: f64) -> Self {
        Self {
            ns: None,
            name,
            kind: Some(DataPointType::Measurement),
            value,
            count: None,
            min: None,
            max: None,
            std_dev: None,
        }
    }

    /// Sets: Namespace of the metric.
    pub fn ns(&mut self, ns: String) -> &mut Self {
        self.ns = Some(ns);
        self
    }

    /// Sets: Metric type. Single measurement or the aggregated value.
    pub fn kind(&mut self, kind: DataPointType) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    /// Sets: Metric weight of the aggregated metric. Should not be set for a measurement.
    pub fn count(&mut self, count: i32) -> &mut Self {
        self.count = Some(count);
        self
    }

    /// Sets: Minimum value of the aggregated metric. Should not be set for a measurement.
    pub fn min(&mut self, min: f64) -> &mut Self {
        self.min = Some(min);
        self
    }

    /// Sets: Maximum value of the aggregated metric. Should not be set for a measurement.
    pub fn max(&mut self, max: f64) -> &mut Self {
        self.max = Some(max);
        self
    }

    /// Sets: Standard deviation of the aggregated metric. Should not be set for a measurement.
    pub fn std_dev(&mut self, std_dev: f64) -> &mut Self {
        self.std_dev = Some(std_dev);
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
