use crate::bond::v2::Visitor;
use crate::bond::{Parser, Schema};
use crate::Result;
use std::fs;
use std::path::Path;

pub struct Compiler;

impl Compiler {
    pub fn compile(input: &Path, output: &Path) -> Result<()> {
        let parser = Parser::new();
        let schema = parser.parse(input)?;

        let module = codegen::Scope::new();

        let mut generator = SchemaGenerator { module };
        generator.visit_schema(&schema);

        fs::write(output, generator.to_string())?;
        Ok(())
    }
}

struct SchemaGenerator {
    module: codegen::Scope,
}

impl SchemaGenerator {
    fn to_string(&self) -> String {
        self.module.to_string()
    }
}

impl Visitor for SchemaGenerator {
    fn visit_schema(&mut self, schema: &Schema) {
        self.module.raw("// NOTE: This file was automatically generated.");
        self.module.import("serde", "Serialize");
        self.module.import("crate::contracts", "*");

        self.visit_declarations(schema.declarations());
    }
}
