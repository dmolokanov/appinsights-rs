use serde::Serialize;

// NOTE: This file was automatically generated.

/// Defines the level of severity for the event.
#[derive(Debug, Serialize)]
pub enum SeverityLevel {
    Verbose,
    Information,
    Warning,
    Error,
    Critical,
}
