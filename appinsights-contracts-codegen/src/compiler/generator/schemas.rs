use crate::ast::{Enum, Schema, Struct};
use crate::compiler::generator::{BuilderGenerator, EnumGenerator, StructGenerator, TelemetryDataTraitGenerator};
use crate::compiler::Visitor;

pub struct SchemaGenerator {
    body: codegen::Scope,
}

impl SchemaGenerator {
    pub fn new() -> Self {
        Self {
            body: codegen::Scope::new(),
        }
    }
}

impl Visitor for SchemaGenerator {
    fn visit_schema(&mut self, schema: &Schema) {
        self.body.raw("// NOTE: This file was automatically generated.");
        self.body.import("serde", "Serialize");
        self.body.import("crate::contracts", "*");

        self.visit_declarations(schema.declarations());
    }

    fn visit_struct(&mut self, declaration: &Struct) {
        // generate struct declaration
        let mut struct_generator = StructGenerator::new(declaration.name());
        struct_generator.visit_struct(declaration);
        struct_generator.push_into(&mut self.body);

        // generate struct builder declaration
        let mut builder_generator = BuilderGenerator::new(declaration.name());
        builder_generator.visit_struct(declaration);
        builder_generator.push_into(&mut self.body);

        // assume that if struct name ends with Data and it is not "Data"
        // so it required TelemetryData trait implemented for this type
        if declaration.is_telemetry_data() {
            let mut telemetry_data_generator = TelemetryDataTraitGenerator::new(declaration.name());
            telemetry_data_generator.visit_struct(declaration);
            telemetry_data_generator.push_into(&mut self.body);
        }
    }

    fn visit_enum(&mut self, declaration: &Enum) {
        let mut enum_generator = EnumGenerator::new(declaration.name());
        enum_generator.visit_enum(declaration);
        enum_generator.push_into(&mut self.body);
    }
}

impl ToString for SchemaGenerator {
    fn to_string(&self) -> String {
        self.body.to_string()
    }
}
