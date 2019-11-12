use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Metric data single measurement.
#[derive(Debug, Clone, Serialize)]
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
    pub fn ns(&mut self, ns: Option<String>) -> &mut Self {
        self.ns = ns;
        self
    }

    /// Sets: Metric type. Single measurement or the aggregated value.
    pub fn kind(&mut self, kind: Option<DataPointType>) -> &mut Self {
        self.kind = kind;
        self
    }

    /// Sets: Metric weight of the aggregated metric. Should not be set for a measurement.
    pub fn count(&mut self, count: Option<i32>) -> &mut Self {
        self.count = count;
        self
    }

    /// Sets: Minimum value of the aggregated metric. Should not be set for a measurement.
    pub fn min(&mut self, min: Option<f64>) -> &mut Self {
        self.min = min;
        self
    }

    /// Sets: Maximum value of the aggregated metric. Should not be set for a measurement.
    pub fn max(&mut self, max: Option<f64>) -> &mut Self {
        self.max = max;
        self
    }

    /// Sets: Standard deviation of the aggregated metric. Should not be set for a measurement.
    pub fn std_dev(&mut self, std_dev: Option<f64>) -> &mut Self {
        self.std_dev = std_dev;
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
