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
    pub fn new(level: impl Into<i32>, method: impl Into<String>) -> Self {
        Self {
            level: level.into(),
            method: method.into(),
            assembly: None,
            file_name: None,
            line: None,
        }
    }

    /// Sets: Name of the assembly (dll, jar, etc.) containing this function.
    pub fn assembly(&mut self, assembly: impl Into<String>) -> &mut Self {
        self.assembly = Some(assembly.into());
        self
    }

    /// Sets: File name or URL of the method implementation.
    pub fn file_name(&mut self, file_name: impl Into<String>) -> &mut Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// Sets: Line number of the code implementation.
    pub fn line(&mut self, line: impl Into<i32>) -> &mut Self {
        self.line = Some(line.into());
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
