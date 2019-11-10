pub mod compiler;

use crate::bond::*;

pub trait Visitor {
    fn visit_schema(&mut self, schema: &Schema) {
        self.visit_declarations(schema.declarations())
    }

    fn visit_declarations(&mut self, declarations: &Vec<UserType>) {
        for declaration in declarations {
            match &declaration {
                UserType::Struct(declaration) => {
                    self.visit_struct(declaration);
                }
                UserType::Enum(declaration) => {
                    self.visit_enum(declaration);
                }
            };
        }
    }

    fn visit_struct(&mut self, declaration: &Struct) {
        if let Some(base) = declaration.base() {
            self.visit_base(base);
        }

        self.visit_fields(declaration.fields());
        self.visit_attributes(declaration.attributes());
    }

    fn visit_base(&mut self, declaration: &Type) {}

    fn visit_fields(&mut self, fields: &Vec<Field>) {
        for field in fields {
            self.visit_field(field);
        }
    }

    fn visit_field(&mut self, field: &Field) {}

    fn visit_attributes(&mut self, attributes: &Vec<Attribute>) {}

    fn visit_enum(&mut self, declaration: &Enum) {}
}
