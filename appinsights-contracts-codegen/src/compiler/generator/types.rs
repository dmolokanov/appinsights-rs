use std::str::FromStr;

use crate::ast::{BasicType, ComplexType, Field, Type, UserType};

impl From<Field> for codegen::Type {
    fn from(field: Field) -> Self {
        let field_type = field.type_().clone();

        if field_type.nullable().is_some() || field.is_required() {
            codegen::Type::from(field_type)
        } else {
            let mut type_ = codegen::Type::new("Option");
            type_.generic(codegen::Type::from(field_type));
            type_
        }
    }
}

impl From<Type> for codegen::Type {
    fn from(type_: Type) -> Self {
        match type_ {
            Type::Basic(type_) => type_.into(),
            Type::Complex(type_) => type_.into(),
        }
    }
}

impl From<BasicType> for codegen::Type {
    fn from(type_: BasicType) -> codegen::Type {
        let name = match type_ {
            BasicType::Bool => "bool",
            BasicType::UInt8 => "u8",
            BasicType::UInt16 => "u16",
            BasicType::UInt32 => "u32",
            BasicType::UInt64 => "u64",
            BasicType::Int8 => "i8",
            BasicType::Int16 => "i16",
            BasicType::Int32 => "i32",
            BasicType::Int64 => "i64",
            BasicType::Float => "f32",
            BasicType::Double => "f64",
            BasicType::String => "String",
            BasicType::WString => "String",
        };

        codegen::Type::new(name)
    }
}

impl From<ComplexType> for codegen::Type {
    fn from(type_: ComplexType) -> codegen::Type {
        match type_ {
            ComplexType::Map { key, element } => {
                let mut type_ = codegen::Type::new("std::collections::HashMap");

                let key = Type::from_str(&key).expect("unexpected type: key");
                type_.generic(key);

                let element = Type::from_str(&element).expect("unexpected type: element");
                type_.generic(element);
                type_
            }
            ComplexType::Parameter { value } => codegen::Type::new(value.name()),
            ComplexType::Vector { element } => {
                let type_: Type = *element;
                type_.into()
            }
            ComplexType::Nullable { element } => {
                let mut type_ = codegen::Type::new("Option");
                let element = *element;
                type_.generic(element);
                type_
            }
            ComplexType::User { declaration } => {
                let name = match *declaration {
                    UserType::Struct(struct_) => struct_.name().to_string(),
                    UserType::Enum(enum_) => enum_.name().to_string(),
                };
                codegen::Type::new(&format!("{}", name))
            }
        }
    }
}
