use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// The abstract common base of all domains.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Domain;

/// Creates: The abstract common base of all domains.
#[derive(Debug, Clone)]
pub struct DomainBuilder;

impl DomainBuilder {
    /// Creates a new [DomainBuilder](trait.DomainBuilder.html) instance with default values set by the schema.
    pub fn new() -> Self {
        Self {}
    }

    /// Creates a new [Domain](trait.Domain.html) instance with values from [DomainBuilder](trait.DomainBuilder.html).
    pub fn build(&self) -> Domain {
        Domain {}
    }
}
