use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Data struct to contain both B and C sections.
#[derive(Debug, Clone, Serialize)]
pub struct Data<TDomain> {
    base_type: Option<String>,
    base_data: TDomain,
}

/// Creates: Data struct to contain both B and C sections.
#[derive(Debug, Clone)]
pub struct DataBuilder<TDomain> {
    base_type: Option<String>,
    base_data: TDomain,
}

impl<TDomain> DataBuilder<TDomain>
where
    TDomain: Clone,
{
    /// Creates a new [DataBuilder](trait.DataBuilder.html) instance with default values set by the schema.
    pub fn new(base_data: TDomain) -> Self {
        Self {
            base_type: None,
            base_data,
        }
    }

    /// Sets: Name of item (B section) if any. If telemetry data is derived straight from this, this should be null.
    pub fn base_type(&mut self, base_type: Option<String>) -> &mut Self {
        self.base_type = base_type;
        self
    }

    /// Creates a new [Data](trait.Data.html) instance with values from [DataBuilder](trait.DataBuilder.html).
    pub fn build(&self) -> Data<TDomain> {
        Data {
            base_type: self.base_type.clone(),
            base_data: self.base_data.clone(),
        }
    }
}
