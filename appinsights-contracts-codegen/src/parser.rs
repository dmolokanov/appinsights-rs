use std::fs::File;
use std::path::Path;

use crate::ast::Schema;
use crate::Result;

#[derive(Default)]
pub struct Parser;

impl Parser {
    pub fn parse(&self, path: &Path) -> Result<Schema> {
        let schema = serde_json::from_reader(File::open(&path)?)?;
        Ok(schema)
    }
}
