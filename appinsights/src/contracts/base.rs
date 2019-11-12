use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Data struct to contain only C section with custom fields.
#[derive(Debug, Clone, Serialize)]
pub struct Base {
    base_type: Option<String>,
}

/// Creates: Data struct to contain only C section with custom fields.
#[derive(Debug, Clone)]
pub struct BaseBuilder {
    base_type: Option<String>,
}

impl BaseBuilder {
    /// Creates a new [BaseBuilder](trait.BaseBuilder.html) instance with default values set by the schema.
    pub fn new() -> Self {
        Self { base_type: None }
    }

    /// Sets: Name of item (B section) if any. If telemetry data is derived straight from this, this should be null.
    pub fn base_type(&mut self, base_type: Option<String>) -> &mut Self {
        self.base_type = base_type;
        self
    }

    /// Creates a new [Base](trait.Base.html) instance with values from [BaseBuilder](trait.BaseBuilder.html).
    pub fn build(&self) -> Base {
        Base {
            base_type: self.base_type.clone(),
        }
    }
}
