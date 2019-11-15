use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of the Metric item is a list of measurements (single data points) and/or aggregations.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricData {
    ver: i32,
    metrics: DataPoint,
    properties: Option<std::collections::BTreeMap<String, String>>,
}

/// Creates: An instance of the Metric item is a list of measurements (single data points) and/or aggregations.
#[derive(Debug, Clone)]
pub struct MetricDataBuilder {
    ver: i32,
    metrics: DataPoint,
    properties: Option<std::collections::BTreeMap<String, String>>,
}

impl MetricDataBuilder {
    /// Creates a new [MetricDataBuilder](trait.MetricDataBuilder.html) instance with default values set by the schema.
    pub fn new(metrics: impl Into<DataPoint>) -> Self {
        Self {
            ver: 2,
            metrics: metrics.into(),
            properties: None,
        }
    }

    /// Sets: Schema version
    pub fn ver(&mut self, ver: impl Into<i32>) -> &mut Self {
        self.ver = ver.into();
        self
    }

    /// Sets: Collection of custom properties.
    pub fn properties(&mut self, properties: impl Into<std::collections::BTreeMap<String, String>>) -> &mut Self {
        self.properties = Some(properties.into());
        self
    }

    /// Creates a new [MetricData](trait.MetricData.html) instance with values from [MetricDataBuilder](trait.MetricDataBuilder.html).
    pub fn build(&self) -> MetricData {
        MetricData {
            ver: self.ver.clone(),
            metrics: self.metrics.clone(),
            properties: self.properties.clone(),
        }
    }
}
