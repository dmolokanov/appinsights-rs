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

/// Creates: Exception details of the exception in a chain.
#[derive(Debug, Clone)]
pub struct ExceptionDetailsBuilder {
    id: Option<i32>,
    outer_id: Option<i32>,
    type_name: String,
    message: String,
    has_full_stack: Option<bool>,
    stack: Option<String>,
    parsed_stack: Option<StackFrame>,
}

impl ExceptionDetailsBuilder {
    /// Creates a new [ExceptionDetailsBuilder](trait.ExceptionDetailsBuilder.html) instance with default values set by the schema.
    pub fn new(type_name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            id: None,
            outer_id: None,
            type_name: type_name.into(),
            message: message.into(),
            has_full_stack: Some(true),
            stack: None,
            parsed_stack: None,
        }
    }

    /// Sets: In case exception is nested (outer exception contains inner one), the id and outerId properties are used to represent the nesting.
    pub fn id(&mut self, id: impl Into<i32>) -> &mut Self {
        self.id = Some(id.into());
        self
    }

    /// Sets: The value of outerId is a reference to an element in ExceptionDetails that represents the outer exception
    pub fn outer_id(&mut self, outer_id: impl Into<i32>) -> &mut Self {
        self.outer_id = Some(outer_id.into());
        self
    }

    /// Sets: Indicates if full exception stack is provided in the exception. The stack may be trimmed, such as in the case of a StackOverflow exception.
    pub fn has_full_stack(&mut self, has_full_stack: impl Into<bool>) -> &mut Self {
        self.has_full_stack = Some(has_full_stack.into());
        self
    }

    /// Sets: Text describing the stack. Either stack or parsedStack should have a value.
    pub fn stack(&mut self, stack: impl Into<String>) -> &mut Self {
        self.stack = Some(stack.into());
        self
    }

    /// Sets: List of stack frames. Either stack or parsedStack should have a value.
    pub fn parsed_stack(&mut self, parsed_stack: impl Into<StackFrame>) -> &mut Self {
        self.parsed_stack = Some(parsed_stack.into());
        self
    }

    /// Creates a new [ExceptionDetails](trait.ExceptionDetails.html) instance with values from [ExceptionDetailsBuilder](trait.ExceptionDetailsBuilder.html).
    pub fn build(&self) -> ExceptionDetails {
        ExceptionDetails {
            id: self.id.clone(),
            outer_id: self.outer_id.clone(),
            type_name: self.type_name.clone(),
            message: self.message.clone(),
            has_full_stack: self.has_full_stack.clone(),
            stack: self.stack.clone(),
            parsed_stack: self.parsed_stack.clone(),
        }
    }
}
