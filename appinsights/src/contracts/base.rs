use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Data struct to contain only C section with custom fields.
#[derive(Debug, Serialize)]
pub struct Base {
    base_type: Option<String>,
}

impl Base {
    /// Create a new [Base](trait.Base.html) instance with default values set by the schema.
    pub fn new() -> Self {
        Self { base_type: None }
    }

    /// Name of item (B section) if any. If telemetry data is derived straight from this, this should be null.
    pub fn with_base_type(&mut self, base_type: Option<String>) -> &mut Self {
        self.base_type = base_type;
        self
    }
}
