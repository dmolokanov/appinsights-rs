use serde::{Deserialize, Serialize};

use crate::ast::{Attribute, Namespace};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Enum {
    enum_constants: Vec<EnumConstant>,
    decl_name: String,
    decl_attributes: Vec<Attribute>,
    decl_namespaces: Vec<Namespace>,
}

impl Enum {
    pub fn name(&self) -> &str {
        &self.decl_name
    }

    pub fn constants(&self) -> &Vec<EnumConstant> {
        &self.enum_constants
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.decl_attributes
    }

    pub fn namespaces(&self) -> &Vec<Namespace> {
        &self.decl_namespaces
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EnumConstant {
    constant_value: Option<String>,
    constant_name: String,
}

impl EnumConstant {
    pub fn name(&self) -> &str {
        &self.constant_name
    }

    pub fn value(&self) -> Option<&String> {
        self.constant_value.as_ref()
    }
}
