use std::fs::File;
use std::path::Path;
use std::str::FromStr;

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Schema {
    pub namespaces: Vec<Namespace>,
    pub imports: Vec<String>,
    pub declarations: Vec<UserTypeDeclaration>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "tag")]
#[serde(deny_unknown_fields)]
pub enum UserTypeDeclaration {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum FieldModifier {
    Optional,
    Required,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum FieldDefault {
    Integer(FieldDefaultValue<i32>),
    Float(FieldDefaultValue<f32>),
    Bool(FieldDefaultValue<bool>),
    String(FieldDefaultValue<String>),
    Enum(FieldDefaultValue<String>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FieldDefaultValue<T> {
    value: T,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum ComplexType {
    Map { key: String, element: String },
    Parameter { value: Parameter },
    Vector { element: Box<Type> },
    Nullable { element: Box<Type> },
    User { declaration: Box<UserTypeDeclaration> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Attribute {
    pub attr_name: Vec<String>,
    pub attr_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    pub param_constraint: Option<String>,
    pub param_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Namespace {
    name: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Enum {
    pub enum_constants: Vec<EnumConstant>,
    pub decl_name: String,
    pub decl_attributes: Vec<Attribute>,
    pub decl_namespaces: Vec<Namespace>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EnumConstant {
    pub constant_value: Option<String>,
    pub constant_name: String,
}
