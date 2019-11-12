mod event;
mod trace;

pub use event::*;
pub use trace::*;

use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A trait that provides Application Insights telemetry items.
pub trait Telemetry {
    /// Returns the time when this telemetry was measured.
    fn timestamp(&self) -> DateTime<Utc>;

    /// Returns custom properties to submit with the telemetry item.
    fn properties(&self) -> &Properties;

    /// Returns custom measurements to submit with the telemetry item.
    fn measurements(&self) -> Option<&Measurements>;

    /// Returns context data containing extra, optional tags. Overrides values found on client telemetry context.
    fn tags(&self) -> &ContextTags;
}

pub type ContextTags = HashMap<String, String>;

pub type Properties = HashMap<String, String>;

pub type Measurements = HashMap<String, f64>;
