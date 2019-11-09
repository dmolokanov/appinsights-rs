use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::bond::{Enum, Struct};

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
                Some(enum_.name())
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
#[serde(tag = "tag")]
#[serde(deny_unknown_fields)]
pub enum UserType {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    param_constraint: Option<String>,
    param_name: String,
}

impl Parameter {
    pub fn constraint(&self) -> &Option<String> {
        &self.param_constraint
    }

    pub fn name(&self) -> &str {
        &self.param_name
    }
}
