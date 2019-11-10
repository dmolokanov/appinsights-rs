use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Data struct to contain both B and C sections.
#[derive(Debug, Serialize)]
pub struct Data<TDomain> {
    base_data: TDomain,
}

impl<TDomain> Data<TDomain> {
    /// Create a new [Data](trait.Data.html) instance with default values set by the schema.
    pub fn new(base_data: TDomain) -> Self {
        Self { base_data }
    }

    /// Container for data item (B section).
    pub fn with_base_data(&mut self, base_data: TDomain) -> &mut Self {
        self.base_data = base_data;
        self
    }
}
