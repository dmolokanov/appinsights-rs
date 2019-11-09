use serde::Serialize;

// NOTE: This file was automatically generated.

/// Stack frame information.
#[derive(Debug, Serialize)]
pub struct StackFrame {
    level: i32,
    method: String,
    assembly: Option<String>,
    file_name: Option<String>,
    line: Option<i32>,
}

impl StackFrame {
    /// Create a new [StackFrame](trait.StackFrame.html) instance with default values set by the schema.
    pub fn new(level: i32, method: String) -> Self {
        Self {
            level,
            method,
            assembly: None,
            file_name: None,
            line: None,
        }
    }

    /// Level in the call stack. For the long stacks SDK may not report every function in a call stack.
    pub fn with_level(&mut self, level: i32) -> &mut Self {
        self.level = level;
        self
    }

    /// Method name.
    pub fn with_method(&mut self, method: String) -> &mut Self {
        self.method = method;
        self
    }

    /// Name of the assembly (dll, jar, etc.) containing this function.
    pub fn with_assembly(&mut self, assembly: Option<String>) -> &mut Self {
        self.assembly = assembly;
        self
    }

    /// File name or URL of the method implementation.
    pub fn with_file_name(&mut self, file_name: Option<String>) -> &mut Self {
        self.file_name = file_name;
        self
    }

    /// Line number of the code implementation.
    pub fn with_line(&mut self, line: Option<i32>) -> &mut Self {
        self.line = line;
        self
    }
}
