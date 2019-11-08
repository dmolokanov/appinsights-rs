use std::fs;
use std::path::Path;

use codegen::Scope;

use crate::bond::*;
use crate::Result;

pub fn generate(schema: &Schema, path: &Path) -> Result<()> {
    let mut compiler = CodeGenerator::new();
    compiler.visit_schema(&schema);
    compiler.save(path)?;

    Ok(())
}

trait Visitor<T> {
    type Value;

    //    fn visit_schema(&self, schema: &Schema) -> Self::Value;
    //    fn visit_user_type_declaration(&self, declaration: &UserTypeDeclaration) -> Self::Value;
    fn visit_schema(&mut self, schema: &Schema);
    fn visit_user_type_declaration(&mut self, declaration: &UserTypeDeclaration);
    fn visit_enum_declaration(&mut self, declaration: &Enum);
}

struct CodeGenerator {
    scope: Scope,
}

impl CodeGenerator {
    fn new() -> Self {
        Self { scope: Scope::new() }
    }

    fn save(&self, path: &Path) -> Result<()> {
        fs::write(path, self.scope.to_string())?;
        Ok(())
    }
}

impl Visitor<Scope> for CodeGenerator {
    type Value = Scope;

    fn visit_schema(&mut self, schema: &Schema) {
        for declaration in schema.declarations.iter() {
            self.visit_user_type_declaration(&declaration);
        }
    }

    fn visit_user_type_declaration(&mut self, declaration: &UserTypeDeclaration) {
        match declaration {
            UserTypeDeclaration::Struct(declaration) => {
                self.scope.new_struct(&declaration.decl_name);
            }
            UserTypeDeclaration::Enum(declaration) => {
                self.visit_enum_declaration(declaration);
            }
        }
    }

    fn visit_enum_declaration(&mut self, declaration: &Enum) {
        let enum_ = self.scope.new_enum(&declaration.decl_name).vis("pub");

        for const_ in declaration.enum_constants.iter() {
            enum_.new_variant(&const_.constant_name);

            if let Some(_) = &const_.constant_value {
                panic!("enum value is not supported: {:#?}", const_)
            }
        }

        for attr in declaration.decl_attributes.iter() {
            if attr.attr_name.iter().any(|name| name == "Description") {
                enum_.doc(&attr.attr_value);
            }
        }
    }
}
