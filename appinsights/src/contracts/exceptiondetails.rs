use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Exception details of the exception in a chain.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails {
    id: Option<i32>,
    outer_id: Option<i32>,
    type_name: String,
    message: String,
    has_full_stack: Option<bool>,
    stack: Option<String>,
    parsed_stack: Option<StackFrame>,
}

impl Default for ExceptionDetails {
    fn default() -> Self {
        Self {
            id: Option::default(),
            outer_id: Option::default(),
            type_name: String::default(),
            message: String::default(),
            has_full_stack: Some(true),
            stack: Option::default(),
            parsed_stack: Option::default(),
        }
    }
}
