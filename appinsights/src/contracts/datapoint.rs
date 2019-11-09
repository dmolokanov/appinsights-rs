use serde::Serialize;

// NOTE: This file was automatically generated.

/// Metric data single measurement.
#[derive(Debug, Serialize)]
pub struct DataPoint {
    ns: Option<String>,
    name: String,
    kind: Option<crate::contracts::DataPointType>,
    value: f64,
    count: Option<i32>,
    min: Option<f64>,
    max: Option<f64>,
    std_dev: Option<f64>,
}

impl DataPoint {
    /// Create a new [DataPoint](trait.DataPoint.html) instance with default values set by the schema.
    pub fn new(name: String, value: f64) -> Self {
        Self {
            ns: None,
            name,
            kind: Some(crate::contracts::DataPointType::Measurement),
            value,
            count: None,
            min: None,
            max: None,
            std_dev: None,
        }
    }

    /// Namespace of the metric.
    pub fn with_ns(&mut self, ns: Option<String>) -> &mut Self {
        self.ns = ns;
        self
    }

    /// Name of the metric.
    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Metric type. Single measurement or the aggregated value.
    pub fn with_kind(&mut self, kind: Option<crate::contracts::DataPointType>) -> &mut Self {
        self.kind = kind;
        self
    }

    /// Single value for measurement. Sum of individual measurements for the aggregation.
    pub fn with_value(&mut self, value: f64) -> &mut Self {
        self.value = value;
        self
    }

    /// Metric weight of the aggregated metric. Should not be set for a measurement.
    pub fn with_count(&mut self, count: Option<i32>) -> &mut Self {
        self.count = count;
        self
    }

    /// Minimum value of the aggregated metric. Should not be set for a measurement.
    pub fn with_min(&mut self, min: Option<f64>) -> &mut Self {
        self.min = min;
        self
    }

    /// Maximum value of the aggregated metric. Should not be set for a measurement.
    pub fn with_max(&mut self, max: Option<f64>) -> &mut Self {
        self.max = max;
        self
    }

    /// Standard deviation of the aggregated metric. Should not be set for a measurement.
    pub fn with_std_dev(&mut self, std_dev: Option<f64>) -> &mut Self {
        self.std_dev = std_dev;
        self
    }
}
