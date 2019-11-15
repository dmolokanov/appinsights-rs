use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Instances of AvailabilityData represent the result of executing an availability test.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityData {
    ver: i32,
    id: String,
    name: String,
    duration: String,
    success: bool,
    run_location: Option<String>,
    message: Option<String>,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

/// Creates: Instances of AvailabilityData represent the result of executing an availability test.
#[derive(Debug, Clone)]
pub struct AvailabilityDataBuilder {
    ver: i32,
    id: String,
    name: String,
    duration: String,
    success: bool,
    run_location: Option<String>,
    message: Option<String>,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl AvailabilityDataBuilder {
    /// Creates a new [AvailabilityDataBuilder](trait.AvailabilityDataBuilder.html) instance with default values set by the schema.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        duration: impl Into<String>,
        success: impl Into<bool>,
    ) -> Self {
        Self {
            ver: 2,
            id: id.into(),
            name: name.into(),
            duration: duration.into(),
            success: success.into(),
            run_location: None,
            message: None,
            properties: None,
            measurements: None,
        }
    }

    /// Sets: Schema version
    pub fn ver(&mut self, ver: impl Into<i32>) -> &mut Self {
        self.ver = ver.into();
        self
    }

    /// Sets: Name of the location where the test was run from.
    pub fn run_location(&mut self, run_location: impl Into<String>) -> &mut Self {
        self.run_location = Some(run_location.into());
        self
    }

    /// Sets: Diagnostic message for the result.
    pub fn message(&mut self, message: impl Into<String>) -> &mut Self {
        self.message = Some(message.into());
        self
    }

    /// Sets: Collection of custom properties.
    pub fn properties(&mut self, properties: impl Into<std::collections::BTreeMap<String, String>>) -> &mut Self {
        self.properties = Some(properties.into());
        self
    }

    /// Sets: Collection of custom measurements.
    pub fn measurements(&mut self, measurements: impl Into<std::collections::BTreeMap<String, f64>>) -> &mut Self {
        self.measurements = Some(measurements.into());
        self
    }

    /// Creates a new [AvailabilityData](trait.AvailabilityData.html) instance with values from [AvailabilityDataBuilder](trait.AvailabilityDataBuilder.html).
    pub fn build(&self) -> AvailabilityData {
        AvailabilityData {
            ver: self.ver.clone(),
            id: self.id.clone(),
            name: self.name.clone(),
            duration: self.duration.clone(),
            success: self.success.clone(),
            run_location: self.run_location.clone(),
            message: self.message.clone(),
            properties: self.properties.clone(),
            measurements: self.measurements.clone(),
        }
    }
}
