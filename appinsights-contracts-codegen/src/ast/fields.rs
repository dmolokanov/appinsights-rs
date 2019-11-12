use serde::{Deserialize, Serialize};

use heck::SnakeCase;

use crate::ast::{Attribute, Type};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Field {
    field_modifier: FieldModifier,
    field_default: Option<FieldDefault>,
    field_type: Type,
    field_name: String,
    field_attributes: Vec<Attribute>,
    field_ordinal: i32,
}

impl Field {
    pub fn is_required(&self) -> bool {
        self.field_modifier == FieldModifier::Required
    }

    pub fn type_(&self) -> &Type {
        &self.field_type
    }

    pub fn name(&self) -> String {
        let name = self.field_name.to_snake_case();
        if RUST_KEYWORDS.contains(&name.as_str()) {
            format!("{}_", name)
        } else {
            name
        }
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.field_attributes
    }

    pub fn default_value(&self) -> Option<String> {
        match (&self.field_default, self.field_type.enum_()) {
            (Some(FieldDefault::Integer { value }), None) => Some(format!("{}", value)),
            (Some(FieldDefault::Float { value }), None) => Some(format!("{}.0", value)),
            (Some(FieldDefault::Bool { value }), None) => Some(format!("{}", value)),
            (Some(FieldDefault::String { value }), None) => Some(format!("String::from(\"{}\")", value)),
            (Some(FieldDefault::Enum { value }), Some(name)) => Some(format!("{}::{}", name, value)),
            (_, Some(_)) => panic!("Unsupported operation"),
            _ => None,
        }
    }
    pub fn is_option(&self) -> bool {
        self.field_type.nullable().is_some() || self.field_modifier == FieldModifier::Optional
    }
}

const RUST_KEYWORDS: [&str; 1] = ["type"];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum FieldModifier {
    Optional,
    Required,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum FieldDefault {
    Integer { value: i32 },
    Float { value: f32 },
    Bool { value: bool },
    String { value: String },
    Enum { value: String },
}
