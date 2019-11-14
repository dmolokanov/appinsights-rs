use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Defines the level of severity for the event.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SeverityLevel {
    Verbose,
    Information,
    Warning,
    Error,
    Critical,
}
