use crate::bond::v2::Visitor;
use crate::bond::{Attribute, Enum, EnumConstant, Parser, Schema, Struct};
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

    //    fn visit_struct(&mut self, declaration: &Struct) {
    //        unimplemented!()
    //    }

    fn visit_enum(&mut self, declaration: &Enum) {
        let mut code = codegen::Enum::new(declaration.name());

        let mut generator = EnumGenerator::new(&mut code);
        generator.visit_enum(declaration);

        self.module.push_enum(code);
    }
}

struct EnumGenerator<'a> {
    code: &'a mut codegen::Enum,
}

impl<'a> EnumGenerator<'a> {
    fn new(code: &'a mut codegen::Enum) -> Self {
        Self { code }
    }
}

impl Visitor for EnumGenerator<'_> {
    fn visit_enum(&mut self, declaration: &Enum) {
        self.code.derive("Debug").derive("Serialize").vis("pub");

        self.visit_enum_constants(declaration.constants());
        self.visit_attributes(declaration.attributes());
    }

    fn visit_enum_constant(&mut self, constant: &EnumConstant) {
        self.code.new_variant(constant.name());

        if let Some(_) = constant.value() {
            panic!("enum value is not supported: {:#?}", constant)
        }
    }

    fn visit_attribute(&mut self, attribute: &Attribute) {
        if attribute.names().iter().any(|name| name == "Description") {
            self.code.doc(attribute.value());
        }
    }
}
