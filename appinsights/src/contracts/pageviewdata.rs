use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of PageView represents a generic action on a page like a button click. It is also the base type for PageView.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageViewData {
    ver: i32,
    name: String,
    url: Option<String>,
    duration: Option<String>,
    referrer_uri: Option<String>,
    id: String,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

/// Creates: An instance of PageView represents a generic action on a page like a button click. It is also the base type for PageView.
#[derive(Debug, Clone)]
pub struct PageViewDataBuilder {
    ver: i32,
    name: String,
    url: Option<String>,
    duration: Option<String>,
    referrer_uri: Option<String>,
    id: String,
    properties: Option<std::collections::BTreeMap<String, String>>,
    measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl PageViewDataBuilder {
    /// Creates a new [PageViewDataBuilder](trait.PageViewDataBuilder.html) instance with default values set by the schema.
    pub fn new(name: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            ver: 2,
            name: name.into(),
            url: None,
            duration: None,
            referrer_uri: None,
            id: id.into(),
            properties: None,
            measurements: None,
        }
    }

    /// Sets: Schema version
    pub fn ver(&mut self, ver: impl Into<i32>) -> &mut Self {
        self.ver = ver.into();
        self
    }

    /// Sets: Request URL with all query string parameters
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }

    /// Sets: Request duration in format: DD.HH:MM:SS.MMMMMM. For a page view (PageViewData), this is the duration. For a page view with performance information (PageViewPerfData), this is the page load time. Must be less than 1000 days.
    pub fn duration(&mut self, duration: impl Into<String>) -> &mut Self {
        self.duration = Some(duration.into());
        self
    }

    /// Sets: Fully qualified page URI or URL of the referring page; if unknown, leave blank
    pub fn referrer_uri(&mut self, referrer_uri: impl Into<String>) -> &mut Self {
        self.referrer_uri = Some(referrer_uri.into());
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

    /// Creates a new [PageViewData](trait.PageViewData.html) instance with values from [PageViewDataBuilder](trait.PageViewDataBuilder.html).
    pub fn build(&self) -> PageViewData {
        PageViewData {
            ver: self.ver.clone(),
            name: self.name.clone(),
            url: self.url.clone(),
            duration: self.duration.clone(),
            referrer_uri: self.referrer_uri.clone(),
            id: self.id.clone(),
            properties: self.properties.clone(),
            measurements: self.measurements.clone(),
        }
    }
}
