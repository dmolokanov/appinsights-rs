use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Instances of Message represent printf-like trace statements that are text-searched. Log4Net, NLog and other text-based log file entries are translated into intances of this type. The message does not have measurements.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageData {
    ver: i32,
    message: String,
    severity_level: Option<SeverityLevel>,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

/// Creates: Instances of Message represent printf-like trace statements that are text-searched. Log4Net, NLog and other text-based log file entries are translated into intances of this type. The message does not have measurements.
#[derive(Debug, Clone)]
pub struct MessageDataBuilder {
    ver: i32,
    message: String,
    severity_level: Option<SeverityLevel>,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl MessageDataBuilder {
    /// Creates a new [MessageDataBuilder](trait.MessageDataBuilder.html) instance with default values set by the schema.
    pub fn new(message: String) -> Self {
        Self {
            ver: 2,
            message,
            severity_level: None,
            properties: None,
            measurements: None,
        }
    }

    /// Sets: Schema version
    pub fn ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Sets: Trace severity level.
    pub fn severity_level(&mut self, severity_level: SeverityLevel) -> &mut Self {
        self.severity_level = Some(severity_level);
        self
    }

    /// Sets: Collection of custom properties.
    pub fn properties(&mut self, properties: impl Into<std::collections::BTreeMap<String, String>>) -> &mut Self {
        self.properties = Some(properties.into());
        self
    }

    /// Sets: Collection of custom measurements.
    pub fn measurements(&mut self, measurements: std::collections::BTreeMap<String, f64>) -> &mut Self {
        self.measurements = Some(measurements);
        self
    }

    /// Creates a new [MessageData](trait.MessageData.html) instance with values from [MessageDataBuilder](trait.MessageDataBuilder.html).
    pub fn build(&self) -> MessageData {
        MessageData {
            ver: self.ver.clone(),
            message: self.message.clone(),
            severity_level: self.severity_level.clone(),
            properties: self.properties.clone(),
            measurements: self.measurements.clone(),
        }
    }
}
