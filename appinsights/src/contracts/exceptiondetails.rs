use crate::contracts::*;
use serde::Serialize;

// NOTE: This file was automatically generated.

/// Exception details of the exception in a chain.
#[derive(Debug, Clone, Serialize)]
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
    pub fn new(type_name: String, message: String) -> Self {
        Self {
            id: None,
            outer_id: None,
            type_name,
            message,
            has_full_stack: Some(true),
            stack: None,
            parsed_stack: None,
        }
    }

    /// Sets: In case exception is nested (outer exception contains inner one), the id and outerId properties are used to represent the nesting.
    pub fn id(&mut self, id: Option<i32>) -> &mut Self {
        self.id = id;
        self
    }

    /// Sets: The value of outerId is a reference to an element in ExceptionDetails that represents the outer exception
    pub fn outer_id(&mut self, outer_id: Option<i32>) -> &mut Self {
        self.outer_id = outer_id;
        self
    }

    /// Sets: Indicates if full exception stack is provided in the exception. The stack may be trimmed, such as in the case of a StackOverflow exception.
    pub fn has_full_stack(&mut self, has_full_stack: Option<bool>) -> &mut Self {
        self.has_full_stack = has_full_stack;
        self
    }

    /// Sets: Text describing the stack. Either stack or parsedStack should have a value.
    pub fn stack(&mut self, stack: Option<String>) -> &mut Self {
        self.stack = stack;
        self
    }

    /// Sets: List of stack frames. Either stack or parsedStack should have a value.
    pub fn parsed_stack(&mut self, parsed_stack: Option<StackFrame>) -> &mut Self {
        self.parsed_stack = parsed_stack;
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
