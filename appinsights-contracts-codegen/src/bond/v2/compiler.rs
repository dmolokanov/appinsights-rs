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
        let mut struct_generator = StructGenerator::new(declaration.name());
        struct_generator.visit_struct(declaration);
        struct_generator.push_into(&mut self.module);

        // generate struct builder declaration
        let mut builder_generator = BuilderGenerator::new(declaration.name());
        builder_generator.visit_struct(declaration);
        builder_generator.push_into(&mut self.module);

        // todo generate implementation of TelemetryData trait
    }

    fn visit_enum(&mut self, declaration: &Enum) {
        let mut enum_generator = EnumGenerator::new(declaration.name());
        enum_generator.visit_enum(declaration);
        enum_generator.push_into(&mut self.module);
    }
}

struct StructGenerator {
    declaration: codegen::Struct,
    generics: HashSet<String>,
    field_names: HashSet<String>,
}

impl StructGenerator {
    fn new(name: &str) -> Self {
        let mut declaration = codegen::Struct::new(&name);
        declaration.derive("Debug").derive("Serialize").vis("pub");

        Self {
            declaration,
            generics: HashSet::default(),
            field_names: HashSet::default(),
        }
    }

    fn push_into(self, module: &mut codegen::Scope) {
        module.push_struct(self.declaration);
    }
}

impl Visitor for StructGenerator {
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
                if !self.generics.contains(generic) {
                    self.declaration.generic(generic);
                }
            }

            // add a field declaration to struct
            let field_type = codegen::Type::from(field.clone());
            self.declaration.field(&field.name(), &field_type);
        }
    }
}

struct BuilderGenerator {
    declaration: codegen::Struct,
    implementation: codegen::Impl,
    constructor: codegen::Function,
    constructor_body: codegen::Block,
    build: codegen::Function,
    build_body: codegen::Block,
    setters: Vec<codegen::Function>,
    generics: HashSet<String>,
    field_names: HashSet<String>,
}

impl BuilderGenerator {
    fn new(name: &str) -> Self {
        let builder_name = format!("{}Builder", name);

        let mut declaration = codegen::Struct::new(&builder_name);
        declaration.vis("pub");

        let implementation = codegen::Impl::new(&builder_name);

        let constructor_doc = format!(
            "Create a new [{builder_name}](trait.{builder_name}.html) instance with default values set by the schema.",
            builder_name = builder_name
        );
        let mut constructor = codegen::Function::new("new");
        constructor.vis("pub").ret("Self").doc(&constructor_doc);

        let constructor_body = codegen::Block::new("Self");

        let build_doc = format!(
            "Create a new [{name}](trait.{name}.html) instance with values from [{builder_name}](trait.{builder_name}.html).",
            name = name, builder_name = builder_name
        );

        let mut build = codegen::Function::new("build");
        build.arg_self().ret(name).doc(&build_doc);

        let build_body = codegen::Block::new(name);

        Self {
            declaration,
            implementation,
            constructor,
            constructor_body,
            build,
            build_body,
            setters: Vec::default(),
            generics: HashSet::default(),
            field_names: HashSet::default(),
        }
    }

    fn push_into(mut self, module: &mut codegen::Scope) {
        module.push_struct(self.declaration);

        self.constructor.push_block(self.constructor_body);
        self.implementation.push_fn(self.constructor);

        for setter in self.setters {
            self.implementation.push_fn(setter);
        }

        self.build.push_block(self.build_body);
        self.implementation.push_fn(self.build);

        module.push_impl(self.implementation);
    }

    fn should_generate_constructor_arg(&self, field: &Field) -> bool {
        field.is_required() && field.default_value().is_none()
    }

    fn should_generate_setter(&self, field: &Field) -> bool {
        !self.should_generate_constructor_arg(field)
    }
}

impl Visitor for BuilderGenerator {
    fn visit_struct_attribute(&mut self, attribute: &Attribute) {
        if attribute.names().iter().any(|name| name == "Description") {
            let doc = format!("Creates: {}", attribute.value());
            self.declaration.doc(&doc);
        }
    }

    fn visit_field(&mut self, field: &Field) {
        // skip duplicating fields
        if self.field_names.insert(field.name()) {
            if let Some(generic) = field.type_().generic() {
                // skip duplicating generic parameters
                if !self.generics.contains(generic) {
                    // add a new generic parameter to builder declaration
                    self.declaration.generic(generic);

                    // add a new generic parameter to builder implementation
                    self.implementation.generic(generic);
                    self.implementation.target_generic(generic);
                }
            }

            // add a field declaration to builder declaration
            let field_name = field.name();
            let field_type = codegen::Type::from(field.clone());

            self.declaration.field(&field_name, &field_type);

            // add constructor arg for required field without default value
            if self.should_generate_constructor_arg(field) {
                self.constructor.arg(&field_name, &field_type);
            }

            // add an line in constructor body to init builder field
            let line = if let Some(value) = field.default_value() {
                let field_value = if field.is_option() {
                    format!("Some({})", value)
                } else {
                    format!("{}", value)
                };
                format!("{}: {},", field_name, field_value)
            } else if field.is_required() {
                // initialize struct field with value from constructor if field is required
                format!("{},", field_name)
            } else {
                // initialize optional field with None
                format!("{}: None,", field_name)
            };
            self.constructor_body.line(line);

            // add a setter for an optional field or
            if self.should_generate_setter(field) {
                let mut setter = codegen::Function::new(&field_name);
                setter
                    .vis("pub")
                    .ret("&mut Self")
                    .arg_mut_self()
                    .arg(&field_name, field_type)
                    .line(format!("self.{name} = {name};", name = field_name))
                    .line("self");

                self.setters.push(setter);

                // collect field attributes only for added setters
                self.visit_field_attributes(field.attributes());
            }

            // populate body block for build method
            self.build_body.line(format!("{},", field_name));
        }
    }

    fn visit_field_attribute(&mut self, attribute: &Attribute) {
        if attribute.names().iter().any(|name| name == "Description") {
            let doc = format!("Sets: {}", attribute.value());
            let setter = self.setters.last_mut().expect("Setter must exist");
            setter.doc(&doc);
        }
    }
}

struct EnumGenerator {
    declaration: codegen::Enum,
}

impl EnumGenerator {
    fn new(name: &str) -> Self {
        let mut declaration = codegen::Enum::new(&name);
        declaration.derive("Debug").derive("Serialize").vis("pub");

        Self { declaration }
    }

    fn push_into(self, module: &mut codegen::Scope) {
        module.push_enum(self.declaration);
    }
}

impl Visitor for EnumGenerator {
    fn visit_enum_constant(&mut self, constant: &EnumConstant) {
        self.declaration.new_variant(constant.name());

        if let Some(_) = constant.value() {
            panic!("enum value is not supported: {:#?}", constant)
        }
    }

    fn visit_enum_attribute(&mut self, attribute: &Attribute) {
        if attribute.names().iter().any(|name| name == "Description") {
            self.declaration.doc(attribute.value());
        }
    }
}
