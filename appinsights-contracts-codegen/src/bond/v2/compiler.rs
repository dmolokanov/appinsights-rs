use crate::bond::v2::Visitor;
use crate::bond::*;
use crate::Result;
use std::collections::HashSet;
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

    fn visit_struct(&mut self, declaration: &Struct) {
        // generate struct declaration
        let mut struct_declaration = codegen::Struct::new(declaration.name());

        let mut struct_generator = StructGenerator::new(&mut struct_declaration);
        struct_generator.visit_struct(declaration);

        self.module.push_struct(struct_declaration);

        // generate struct builder declaration
        let builder_name = format!("{}Builder", declaration.name());
        let mut builder_declaration = codegen::Struct::new(&builder_name);
        let mut builder_implementation = codegen::Impl::new(&builder_name);

        let mut builder_generator = BuilderGenerator::new(&mut builder_declaration, &mut builder_implementation);
        builder_generator.visit_struct(declaration);

        self.module.push_struct(builder_declaration);
        self.module.push_impl(builder_implementation);

        // todo generate implementation of TelemetryData trait
    }

    fn visit_enum(&mut self, declaration: &Enum) {
        let enumeration = self.module.new_enum(declaration.name());

        let mut generator = EnumGenerator::new(enumeration);
        generator.visit_enum(declaration);
    }
}

struct StructGenerator<'a> {
    declaration: &'a mut codegen::Struct,
    generics: HashSet<&'a str>,
    field_names: HashSet<String>,
}

impl<'a> StructGenerator<'a> {
    fn new(declaration: &'a mut codegen::Struct) -> Self {
        Self {
            declaration: declaration.derive("Debug").derive("Serialize").vis("pub"),
            generics: HashSet::default(),
            field_names: HashSet::default(),
        }
    }
}

impl Visitor for StructGenerator<'_> {
    fn visit_struct_attribute(&mut self, attribute: &Attribute) {
        if attribute.names().iter().any(|name| name == "Description") {
            self.declaration.doc(attribute.value());
        }
    }

    fn visit_field(&mut self, field: &Field) {
        // skip duplicating fields
        if self.field_names.insert(field.name()) {
            // add a new generic parameter to struct declaration
            if let Some(generic) = field.type_().generic() {
                if !self.generics.contains(&generic) {
                    self.declaration.generic(generic);
                }
            }

            // add a field declaration to struct
            let field_type = codegen::Type::from(field.clone());
            self.declaration.field(&field.name(), &field_type);
        }
    }
}

struct BuilderGenerator<'a> {
    declaration: &'a mut codegen::Struct,
    implementation: &'a mut codegen::Impl,
    constructor: &'a mut codegen::Function,
    setters: Vec<codegen::Function>,
    generics: HashSet<&'a str>,
    field_names: HashSet<String>,
}

impl<'a> BuilderGenerator<'a> {
    fn new(declaration: &'a mut codegen::Struct, implementation: &'a mut codegen::Impl) -> Self {
        let mut constructor = codegen::Function::new("new");
        implementation.push_fn(constructor);
        Self {
            declaration: declaration.vis("pub"),
            implementation,
            constructor: &mut constructor,
            setters: Vec::default(),
            generics: HashSet::default(),
            field_names: HashSet::default(),
        }
    }
}

impl Visitor for BuilderGenerator<'_> {
    fn visit_struct_attribute(&mut self, attribute: &Attribute) {
        if attribute.names().iter().any(|name| name == "Description") {
            let doc = format!("Creates an instance of: {}", attribute.value());
            self.declaration.doc(&doc);
        }
    }

    fn visit_field(&mut self, field: &Field) {
        // skip duplicating fields
        if self.field_names.insert(field.name()) {
            if let Some(generic) = field.type_().generic() {
                // skip duplicating generic parameters
                if !self.generics.contains(&generic) {
                    // add a new generic parameter to builder declaration
                    self.declaration.generic(generic);

                    // add a new generic parameter to builder implementation
                    self.implementation.generic(generic);
                }
            }

            // add a field declaration to builder declaration
            let field_type = codegen::Type::from(field.clone());
            self.declaration.field(&field.name(), &field_type);

            //
        }
    }
}

struct EnumGenerator<'a> {
    code: &'a mut codegen::Enum,
}

impl<'a> EnumGenerator<'a> {
    fn new(code: &'a mut codegen::Enum) -> Self {
        Self {
            code: code.derive("Debug").derive("Serialize").vis("pub"),
        }
    }
}

impl Visitor for EnumGenerator<'_> {
    fn visit_enum_constant(&mut self, constant: &EnumConstant) {
        self.code.new_variant(constant.name());

        if let Some(_) = constant.value() {
            panic!("enum value is not supported: {:#?}", constant)
        }
    }

    fn visit_enum_attribute(&mut self, attribute: &Attribute) {
        if attribute.names().iter().any(|name| name == "Description") {
            self.code.doc(attribute.value());
        }
    }
}
