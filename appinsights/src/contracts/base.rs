use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Data struct to contain only C section with custom fields.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Base {
    Data(Data),
}
