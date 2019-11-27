use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// An instance of PageView represents a generic action on a page like a button click. It is also the base type for PageView.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageViewData {
    pub ver: i32,
    pub name: String,
    pub url: Option<String>,
    pub duration: Option<String>,
    pub referrer_uri: Option<String>,
    pub id: String,
    pub properties: Option<std::collections::BTreeMap<String, String>>,
    pub measurements: Option<std::collections::BTreeMap<String, f64>>,
}

impl Default for PageViewData {
    fn default() -> Self {
        Self {
            ver: 2,
            name: String::default(),
            url: Option::default(),
            duration: Option::default(),
            referrer_uri: Option::default(),
            id: String::default(),
            properties: Option::default(),
            measurements: Option::default(),
        }
    }
}
