use crate::bond::*;

#[allow(unused_variables)]
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

        self.visit_struct_attributes(declaration.attributes());
        self.visit_fields(declaration.fields());
    }

    fn visit_struct_attributes(&mut self, attributes: &Vec<Attribute>) {
        for attribute in attributes {
            self.visit_struct_attribute(attribute);
        }
    }

    fn visit_struct_attribute(&mut self, attribute: &Attribute) {}

    fn visit_base(&mut self, declaration: &Type) {
        if let Type::Complex(ComplexType::User { declaration }) = &declaration {
            if let UserType::Struct(declaration) = &**declaration {
                self.visit_struct(declaration);
            } else {
                panic!(format!("Unsupported struct base type: {:?}", declaration))
            }
        } else {
            panic!(format!("Unsupported struct base type: {:?}", declaration))
        }
    }

    fn visit_fields(&mut self, fields: &Vec<Field>) {
        for field in fields {
            self.visit_field(field);
        }
    }

    fn visit_field(&mut self, field: &Field) {
        self.visit_field_attributes(field.attributes());
    }

    fn visit_field_attributes(&mut self, attributes: &Vec<Attribute>) {
        for attribute in attributes {
            self.visit_field_attribute(attribute);
        }
    }

    fn visit_field_attribute(&mut self, attribute: &Attribute) {}

    fn visit_enum(&mut self, declaration: &Enum) {
        self.visit_enum_constants(declaration.constants());
        self.visit_enum_attributes(declaration.attributes());
    }

    fn visit_enum_constants(&mut self, constants: &Vec<EnumConstant>) {
        for constant in constants {
            self.visit_enum_constant(constant);
        }
    }

    fn visit_enum_constant(&mut self, constant: &EnumConstant) {}

    fn visit_enum_attributes(&mut self, attributes: &Vec<Attribute>) {
        for attribute in attributes {
            self.visit_enum_attribute(attribute);
        }
    }

    fn visit_enum_attribute(&mut self, attribute: &Attribute) {}
}
