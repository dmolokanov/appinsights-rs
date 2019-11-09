use serde::Serialize;

// NOTE: This file was automatically generated.

/// Exception details of the exception in a chain.
#[derive(Debug, Serialize)]
pub struct ExceptionDetails {
    id: Option<i32>,
    outer_id: Option<i32>,
    type_name: String,
    message: String,
    has_full_stack: Option<bool>,
    stack: Option<String>,
    parsed_stack: Option<crate::contracts::StackFrame>,
}

impl ExceptionDetails {
    /// Create a new [ExceptionDetails](trait.ExceptionDetails.html) instance with default values set by the schema.
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

    /// In case exception is nested (outer exception contains inner one), the id and outerId properties are used to represent the nesting.
    pub fn with_id(&mut self, id: Option<i32>) -> &mut Self {
        self.id = id;
        self
    }

    /// The value of outerId is a reference to an element in ExceptionDetails that represents the outer exception
    pub fn with_outer_id(&mut self, outer_id: Option<i32>) -> &mut Self {
        self.outer_id = outer_id;
        self
    }

    /// Exception type name.
    pub fn with_type_name(&mut self, type_name: String) -> &mut Self {
        self.type_name = type_name;
        self
    }

    /// Exception message.
    pub fn with_message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }

    /// Indicates if full exception stack is provided in the exception. The stack may be trimmed, such as in the case of a StackOverflow exception.
    pub fn with_has_full_stack(&mut self, has_full_stack: Option<bool>) -> &mut Self {
        self.has_full_stack = has_full_stack;
        self
    }

    /// Text describing the stack. Either stack or parsedStack should have a value.
    pub fn with_stack(&mut self, stack: Option<String>) -> &mut Self {
        self.stack = stack;
        self
    }

    /// List of stack frames. Either stack or parsedStack should have a value.
    pub fn with_parsed_stack(&mut self, parsed_stack: Option<crate::contracts::StackFrame>) -> &mut Self {
        self.parsed_stack = parsed_stack;
        self
    }
}
