use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of PageView represents a generic action on a page like a button click. It is also the base type for PageView.
#[derive(Debug, Serialize)]
pub struct PageViewData {
    ver: i32,
    name: String,
    url: Option<String>,
    duration: Option<String>,
    referrer_uri: Option<String>,
    id: String,
    properties: Option<std::collections::HashMap<String, String>>,
    measurements: Option<std::collections::HashMap<String, f64>>,
}

impl PageViewData {
    /// Create a new [PageViewData](trait.PageViewData.html) instance with default values set by the schema.
    pub fn new(name: String, id: String) -> Self {
        Self {
            ver: 2,
            name,
            url: None,
            duration: None,
            referrer_uri: None,
            id,
            properties: None,
            measurements: None,
        }
    }

    /// Schema version
    pub fn with_ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Event name. Keep it low cardinality to allow proper grouping and useful metrics.
    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Request URL with all query string parameters
    pub fn with_url(&mut self, url: Option<String>) -> &mut Self {
        self.url = url;
        self
    }

    /// Request duration in format: DD.HH:MM:SS.MMMMMM. For a page view (PageViewData), this is the duration. For a page view with performance information (PageViewPerfData), this is the page load time. Must be less than 1000 days.
    pub fn with_duration(&mut self, duration: Option<String>) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Fully qualified page URI or URL of the referring page; if unknown, leave blank
    pub fn with_referrer_uri(&mut self, referrer_uri: Option<String>) -> &mut Self {
        self.referrer_uri = referrer_uri;
        self
    }

    /// Identifier of a page view instance. Used for correlation between page view and other telemetry items.
    pub fn with_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// Collection of custom properties.
    pub fn with_properties(&mut self, properties: Option<std::collections::HashMap<String, String>>) -> &mut Self {
        self.properties = properties;
        self
    }

    /// Collection of custom measurements.
    pub fn with_measurements(&mut self, measurements: Option<std::collections::HashMap<String, f64>>) -> &mut Self {
        self.measurements = measurements;
        self
    }
}
