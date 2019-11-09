use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Namespace {
    name: Vec<String>,
}

impl Namespace {
    pub fn names(&self) -> &Vec<String> {
        &self.name
    }
}
