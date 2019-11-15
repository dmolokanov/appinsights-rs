use crate::ast::{Attribute, EnumConstant};
use crate::compiler::Visitor;

pub struct EnumGenerator {
    declaration: codegen::Enum,
}

impl EnumGenerator {
    pub fn new(name: &str) -> Self {
        let mut declaration = codegen::Enum::new(&name);
        declaration
            .derive("Debug")
            .derive("Clone")
            .derive("Serialize")
            .vis("pub");

        Self { declaration }
    }

    pub fn push_into(self, module: &mut codegen::Scope) {
        module.push_enum(self.declaration);
    }
}

impl Visitor for EnumGenerator {
    fn visit_enum_constant(&mut self, constant: &EnumConstant) {
        self.declaration.new_variant(constant.name());

        if constant.value().is_some() {
            panic!("enum value is not supported: {:#?}", constant)
        }
    }

    fn visit_enum_attribute(&mut self, attribute: &Attribute) {
        if attribute.names().iter().any(|name| name == "Description") {
            self.declaration.doc(attribute.value());
        }
    }
}
