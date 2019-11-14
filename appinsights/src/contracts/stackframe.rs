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

/// Creates: Stack frame information.
#[derive(Debug, Clone)]
pub struct StackFrameBuilder {
    level: i32,
    method: String,
    assembly: Option<String>,
    file_name: Option<String>,
    line: Option<i32>,
}

impl StackFrameBuilder {
    /// Creates a new [StackFrameBuilder](trait.StackFrameBuilder.html) instance with default values set by the schema.
    pub fn new(level: i32, method: String) -> Self {
        Self {
            level,
            method,
            assembly: None,
            file_name: None,
            line: None,
        }
    }

    /// Sets: Name of the assembly (dll, jar, etc.) containing this function.
    pub fn assembly(&mut self, assembly: String) -> &mut Self {
        self.assembly = Some(assembly);
        self
    }

    /// Sets: File name or URL of the method implementation.
    pub fn file_name(&mut self, file_name: String) -> &mut Self {
        self.file_name = Some(file_name);
        self
    }

    /// Sets: Line number of the code implementation.
    pub fn line(&mut self, line: i32) -> &mut Self {
        self.line = Some(line);
        self
    }

    /// Creates a new [StackFrame](trait.StackFrame.html) instance with values from [StackFrameBuilder](trait.StackFrameBuilder.html).
    pub fn build(&self) -> StackFrame {
        StackFrame {
            level: self.level.clone(),
            method: self.method.clone(),
            assembly: self.assembly.clone(),
            file_name: self.file_name.clone(),
            line: self.line.clone(),
        }
    }
}
