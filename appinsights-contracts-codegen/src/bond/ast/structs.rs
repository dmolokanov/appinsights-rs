use serde::{Deserialize, Serialize};

use crate::bond::{Attribute, Field, Namespace, Parameter, Type};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Struct {
    struct_base: Option<Type>,
    struct_fields: Vec<Field>,
    decl_namespaces: Vec<Namespace>,
    decl_params: Vec<Parameter>,
    decl_name: String,
    decl_attributes: Vec<Attribute>,
}

impl Struct {
    pub fn base(&self) -> &Option<Type> {
        &self.struct_base
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.struct_fields
    }

    pub fn namespaces(&self) -> &Vec<Namespace> {
        &self.decl_namespaces
    }

    pub fn params(&self) -> &Vec<Parameter> {
        &self.decl_params
    }

    pub fn name(&self) -> &str {
        &self.decl_name
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.decl_attributes
    }

    pub fn is_telemetry_data(&self) -> bool {
        self.name().ends_with("Data") && self.name().len() > 4
    }
}
