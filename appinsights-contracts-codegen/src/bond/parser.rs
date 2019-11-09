use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use heck::SnakeCase;
use serde::{Deserialize, Serialize};

use crate::Result;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, path: &Path) -> Result<Schema> {
        let schema = serde_json::from_reader(File::open(&path)?)?;
        Ok(schema)
    }
}

pub fn parse(path: &Path) -> Result<Schema> {
    let schema = serde_json::from_reader(File::open(&path)?)?;
    Ok(schema)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Schema {
    pub namespaces: Vec<Namespace>,
    pub imports: Vec<String>,
    pub declarations: Vec<UserType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "tag")]
#[serde(deny_unknown_fields)]
pub enum UserType {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Struct {
    pub struct_base: Option<Type>,
    pub struct_fields: Vec<Field>,
    pub decl_namespaces: Vec<Namespace>,
    pub decl_params: Vec<Parameter>,
    pub decl_name: String,
    pub decl_attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub field_modifier: FieldModifier,
    pub field_default: Option<FieldDefault>,
    pub field_type: Type,
    pub field_name: String,
    pub field_attributes: Vec<Attribute>,
    pub field_ordinal: i32,
}

impl Field {
    pub fn default_value(&self) -> Option<String> {
        match (&self.field_default, self.field_type.enum_()) {
            (Some(FieldDefault::Integer { value }), None) => Some(format!("{}", value)),
            (Some(FieldDefault::Float { value }), None) => Some(format!("{}.0", value)),
            (Some(FieldDefault::Bool { value }), None) => Some(format!("{}", value)),
            (Some(FieldDefault::String { value }), None) => Some(format!("String::from(\"{}\")", value)),
            (Some(FieldDefault::Enum { value }), Some(name)) => Some(format!("crate::contracts::{}::{}", name, value)),
            (_, Some(_)) => panic!("Unsupported operation"),
            _ => None,
        }
    }
    pub fn is_option(&self) -> bool {
        self.field_type.nullable().is_some() || self.field_modifier == FieldModifier::Optional
    }

    pub fn name(&self) -> String {
        let name = self.field_name.to_snake_case();
        if RUST_KEYWORDS.contains(&name.as_str()) {
            format!("{}_", name)
        } else {
            name
        }
    }
}

const RUST_KEYWORDS: [&'static str; 1] = ["type"];

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum Type {
    Basic(BasicType),
    Complex(ComplexType),
}

impl Type {
    pub fn nullable(&self) -> Option<&Type> {
        if let Type::Complex(ComplexType::Nullable { element }) = &self {
            Some(element)
        } else {
            None
        }
    }

    pub fn generic(&self) -> Option<&str> {
        if let Type::Complex(ComplexType::Parameter { value }) = &self {
            Some(&value.param_name)
        } else {
            None
        }
    }

    pub fn enum_(&self) -> Option<&str> {
        if let Type::Complex(ComplexType::User { declaration }) = &self {
            if let UserType::Enum(enum_) = &**declaration {
                Some(&enum_.decl_name)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "double" => Ok(Self::Basic(BasicType::Double)),
            "string" => Ok(Self::Basic(BasicType::String)),
            _ => Err(format!("Unsupported type: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
pub enum BasicType {
    Bool,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    String,
    WString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum ComplexType {
    Map { key: String, element: String },
    Parameter { value: Parameter },
    Vector { element: Box<Type> },
    Nullable { element: Box<Type> },
    User { declaration: Box<UserType> },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Attribute {
    pub attr_name: Vec<String>,
    pub attr_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    pub param_constraint: Option<String>,
    pub param_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Namespace {
    name: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Enum {
    pub enum_constants: Vec<EnumConstant>,
    pub decl_name: String,
    pub decl_attributes: Vec<Attribute>,
    pub decl_namespaces: Vec<Namespace>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EnumConstant {
    pub constant_value: Option<String>,
    pub constant_name: String,
}
