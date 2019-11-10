use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// System variables for a telemetry item.
#[derive(Debug, Serialize)]
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

impl Envelope {
    /// Create a new [Envelope](trait.Envelope.html) instance with default values set by the schema.
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

    /// Envelope version. For internal use only. By assigning this the default, it will not be serialized within the payload unless changed to a value other than #1.
    pub fn with_ver(&mut self, ver: Option<i32>) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Type name of telemetry data item.
    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Event date time when telemetry item was created. This is the wall clock time on the client when the event was generated. There is no guarantee that the client's time is accurate. This field must be formatted in UTC ISO 8601 format, with a trailing 'Z' character, as described publicly on https://en.wikipedia.org/wiki/ISO_8601#UTC. Note: the number of decimal seconds digits provided are variable (and unspecified). Consumers should handle this, i.e. managed code consumers should not use format 'O' for parsing as it specifies a fixed length. Example: 2009-06-15T13:45:30.0000000Z.
    pub fn with_time(&mut self, time: String) -> &mut Self {
        self.time = time;
        self
    }

    /// Sampling rate used in application. This telemetry item represents 1 / sampleRate actual telemetry items.
    pub fn with_sample_rate(&mut self, sample_rate: Option<f64>) -> &mut Self {
        self.sample_rate = sample_rate;
        self
    }

    /// Sequence field used to track absolute order of uploaded events.
    pub fn with_seq(&mut self, seq: Option<String>) -> &mut Self {
        self.seq = seq;
        self
    }

    /// The application's instrumentation key. The key is typically represented as a GUID, but there are cases when it is not a guid. No code should rely on iKey being a GUID. Instrumentation key is case insensitive.
    pub fn with_i_key(&mut self, i_key: Option<String>) -> &mut Self {
        self.i_key = i_key;
        self
    }

    /// A collection of values bit-packed to represent how the event was processed. Currently represents whether IP address needs to be stripped out from event (set 0x200000) or should be preserved.
    pub fn with_flags(&mut self, flags: Option<i64>) -> &mut Self {
        self.flags = flags;
        self
    }

    /// Key/value collection of context properties. See ContextTagKeys for information on available properties.
    pub fn with_tags(&mut self, tags: Option<std::collections::HashMap<String, String>>) -> &mut Self {
        self.tags = tags;
        self
    }

    /// Telemetry data item.
    pub fn with_data(&mut self, data: Option<Base>) -> &mut Self {
        self.data = data;
        self
    }
}
