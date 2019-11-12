use serde::{Deserialize, Serialize};

use crate::ast::{Namespace, UserType};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Schema {
    namespaces: Vec<Namespace>,
    imports: Vec<String>,
    declarations: Vec<UserType>,
}

impl Schema {
    pub fn imports(&self) -> &Vec<String> {
        &self.imports
    }

    pub fn namespaces(&self) -> &Vec<Namespace> {
        &self.namespaces
    }

    pub fn declarations(&self) -> &Vec<UserType> {
        &self.declarations
    }
}
