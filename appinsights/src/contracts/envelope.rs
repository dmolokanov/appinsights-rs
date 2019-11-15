use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// System variables for a telemetry item.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Envelope {
    ver: Option<i32>,
    name: String,
    time: String,
    sample_rate: Option<f64>,
    seq: Option<String>,
    i_key: Option<String>,
    flags: Option<i64>,
    tags: Option<std::collections::BTreeMap<String, String>>,
    data: Option<Base>,
}

/// Creates: System variables for a telemetry item.
#[derive(Debug, Clone)]
pub struct EnvelopeBuilder {
    ver: Option<i32>,
    name: String,
    time: String,
    sample_rate: Option<f64>,
    seq: Option<String>,
    i_key: Option<String>,
    flags: Option<i64>,
    tags: Option<std::collections::BTreeMap<String, String>>,
    data: Option<Base>,
}

impl EnvelopeBuilder {
    /// Creates a new [EnvelopeBuilder](trait.EnvelopeBuilder.html) instance with default values set by the schema.
    pub fn new(name: impl Into<String>, time: impl Into<String>) -> Self {
        Self {
            ver: Some(1),
            name: name.into(),
            time: time.into(),
            sample_rate: Some(100.0),
            seq: None,
            i_key: None,
            flags: None,
            tags: None,
            data: None,
        }
    }

    /// Sets: Envelope version. For internal use only. By assigning this the default, it will not be serialized within the payload unless changed to a value other than #1.
    pub fn ver(&mut self, ver: impl Into<i32>) -> &mut Self {
        self.ver = Some(ver.into());
        self
    }

    /// Sets: Sampling rate used in application. This telemetry item represents 1 / sampleRate actual telemetry items.
    pub fn sample_rate(&mut self, sample_rate: impl Into<f64>) -> &mut Self {
        self.sample_rate = Some(sample_rate.into());
        self
    }

    /// Sets: Sequence field used to track absolute order of uploaded events.
    pub fn seq(&mut self, seq: impl Into<String>) -> &mut Self {
        self.seq = Some(seq.into());
        self
    }

    /// Sets: The application's instrumentation key. The key is typically represented as a GUID, but there are cases when it is not a guid. No code should rely on iKey being a GUID. Instrumentation key is case insensitive.
    pub fn i_key(&mut self, i_key: impl Into<String>) -> &mut Self {
        self.i_key = Some(i_key.into());
        self
    }

    /// Sets: A collection of values bit-packed to represent how the event was processed. Currently represents whether IP address needs to be stripped out from event (set 0x200000) or should be preserved.
    pub fn flags(&mut self, flags: impl Into<i64>) -> &mut Self {
        self.flags = Some(flags.into());
        self
    }

    /// Sets: Key/value collection of context properties. See ContextTagKeys for information on available properties.
    pub fn tags(&mut self, tags: impl Into<std::collections::BTreeMap<String, String>>) -> &mut Self {
        self.tags = Some(tags.into());
        self
    }

    /// Sets: Telemetry data item.
    pub fn data(&mut self, data: impl Into<Base>) -> &mut Self {
        self.data = Some(data.into());
        self
    }

    /// Creates a new [Envelope](trait.Envelope.html) instance with values from [EnvelopeBuilder](trait.EnvelopeBuilder.html).
    pub fn build(&self) -> Envelope {
        Envelope {
            ver: self.ver.clone(),
            name: self.name.clone(),
            time: self.time.clone(),
            sample_rate: self.sample_rate.clone(),
            seq: self.seq.clone(),
            i_key: self.i_key.clone(),
            flags: self.flags.clone(),
            tags: self.tags.clone(),
            data: self.data.clone(),
        }
    }
}
