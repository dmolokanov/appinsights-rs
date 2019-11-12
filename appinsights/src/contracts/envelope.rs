use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// System variables for a telemetry item.
#[derive(Debug, Clone, Serialize)]
pub struct Envelope {
    ver: Option<i32>,
    name: String,
    time: String,
    sample_rate: Option<f64>,
    seq: Option<String>,
    i_key: Option<String>,
    flags: Option<i64>,
    tags: Option<std::collections::HashMap<String, String>>,
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
    tags: Option<std::collections::HashMap<String, String>>,
    data: Option<Base>,
}

impl EnvelopeBuilder {
    /// Creates a new [EnvelopeBuilder](trait.EnvelopeBuilder.html) instance with default values set by the schema.
    pub fn new(name: String, time: String) -> Self {
        Self {
            ver: Some(1),
            name,
            time,
            sample_rate: Some(100.0),
            seq: None,
            i_key: None,
            flags: None,
            tags: None,
            data: None,
        }
    }

    /// Sets: Envelope version. For internal use only. By assigning this the default, it will not be serialized within the payload unless changed to a value other than #1.
    pub fn ver(&mut self, ver: Option<i32>) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Sets: Sampling rate used in application. This telemetry item represents 1 / sampleRate actual telemetry items.
    pub fn sample_rate(&mut self, sample_rate: Option<f64>) -> &mut Self {
        self.sample_rate = sample_rate;
        self
    }

    /// Sets: Sequence field used to track absolute order of uploaded events.
    pub fn seq(&mut self, seq: Option<String>) -> &mut Self {
        self.seq = seq;
        self
    }

    /// Sets: The application's instrumentation key. The key is typically represented as a GUID, but there are cases when it is not a guid. No code should rely on iKey being a GUID. Instrumentation key is case insensitive.
    pub fn i_key(&mut self, i_key: Option<String>) -> &mut Self {
        self.i_key = i_key;
        self
    }

    /// Sets: A collection of values bit-packed to represent how the event was processed. Currently represents whether IP address needs to be stripped out from event (set 0x200000) or should be preserved.
    pub fn flags(&mut self, flags: Option<i64>) -> &mut Self {
        self.flags = flags;
        self
    }

    /// Sets: Key/value collection of context properties. See ContextTagKeys for information on available properties.
    pub fn tags(&mut self, tags: Option<std::collections::HashMap<String, String>>) -> &mut Self {
        self.tags = tags;
        self
    }

    /// Sets: Telemetry data item.
    pub fn data(&mut self, data: Option<Base>) -> &mut Self {
        self.data = data;
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
