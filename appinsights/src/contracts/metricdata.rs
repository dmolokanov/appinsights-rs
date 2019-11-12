use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of the Metric item is a list of measurements (single data points) and/or aggregations.
#[derive(Debug, Clone, Serialize)]
pub struct MetricData {
    ver: i32,
    metrics: DataPoint,
    properties: Option<std::collections::HashMap<String, String>>,
}

/// Creates: An instance of the Metric item is a list of measurements (single data points) and/or aggregations.
#[derive(Debug, Clone)]
pub struct MetricDataBuilder {
    ver: i32,
    metrics: DataPoint,
    properties: Option<std::collections::HashMap<String, String>>,
}

impl MetricDataBuilder {
    /// Creates a new [MetricDataBuilder](trait.MetricDataBuilder.html) instance with default values set by the schema.
    pub fn new(metrics: DataPoint) -> Self {
        Self {
            ver: 2,
            metrics,
            properties: None,
        }
    }

    /// Sets: Schema version
    pub fn ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Sets: Collection of custom properties.
    pub fn properties(&mut self, properties: std::collections::HashMap<String, String>) -> &mut Self {
        self.properties = Some(properties);
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

impl TelemetryData for MetricData {
    /// Returns the base type when placed within an [Data](trait.Data.html) container.
    fn base_type(&self) -> String {
        String::from("MetricData")
    }
}
