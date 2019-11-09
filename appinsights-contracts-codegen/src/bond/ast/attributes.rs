use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Attribute {
    attr_name: Vec<String>,
    attr_value: String,
}

impl Attribute {
    pub fn names(&self) -> &Vec<String> {
        &self.attr_name
    }

    pub fn value(&self) -> &str {
        &self.attr_value
    }
}
