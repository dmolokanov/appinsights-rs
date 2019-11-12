use std::fs::File;
use std::path::Path;

use crate::ast::Schema;
use crate::Result;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, path: &Path) -> Result<Schema> {
        let schema = serde_json::from_reader(File::open(&path)?)?;
        Ok(schema)
    }
}
