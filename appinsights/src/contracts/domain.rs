use serde::Serialize;

// NOTE: This file was automatically generated.

/// The abstract common base of all domains.
#[derive(Debug, Serialize)]
pub struct Domain;

impl Domain {
    /// Create a new [Domain](trait.Domain.html) instance with default values set by the schema.
    pub fn new() -> Self {
        Self {}
    }
}
