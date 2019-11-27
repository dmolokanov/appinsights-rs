use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Stack frame information.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackFrame {
    level: i32,
    method: String,
    assembly: Option<String>,
    file_name: Option<String>,
    line: Option<i32>,
}

impl Default for StackFrame {
    fn default() -> Self {
        Self {
            level: i32::default(),
            method: String::default(),
            assembly: Option::default(),
            file_name: Option::default(),
            line: Option::default(),
        }
    }
}
