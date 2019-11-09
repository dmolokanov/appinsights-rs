use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of the Metric item is a list of measurements (single data points) and/or aggregations.
#[derive(Debug, Serialize)]
pub struct MetricData {
    ver: i32,
    metrics: crate::contracts::DataPoint,
    properties: Option<std::collections::HashMap<String, String>>,
}

impl MetricData {
    /// Create a new [MetricData](trait.MetricData.html) instance with default values set by the schema.
    pub fn new(metrics: crate::contracts::DataPoint) -> Self {
        Self {
            ver: 2,
            metrics,
            properties: None,
        }
    }

    /// Schema version
    pub fn with_ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// List of metrics. Only one metric in the list is currently supported by Application Insights storage. If multiple data points were sent only the first one will be used.
    pub fn with_metrics(&mut self, metrics: crate::contracts::DataPoint) -> &mut Self {
        self.metrics = metrics;
        self
    }

    /// Collection of custom properties.
    pub fn with_properties(&mut self, properties: Option<std::collections::HashMap<String, String>>) -> &mut Self {
        self.properties = properties;
        self
    }
}
