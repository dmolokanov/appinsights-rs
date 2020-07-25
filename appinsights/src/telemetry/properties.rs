use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

/// Contains all properties for telemetry to submit.
#[derive(Debug, Clone, Default)]
pub struct Properties(BTreeMap<String, String>);

impl Properties {
    /// Combines all properties from two objects. It can override some properties with values found
    /// in the second properties bag.
    pub fn combine(a: Properties, b: Properties) -> Self {
        let items = a.0.into_iter().chain(b.0).collect();
        Self(items)
    }
}

impl From<Properties> for BTreeMap<String, String> {
    fn from(properties: Properties) -> Self {
        properties.0
    }
}

impl Deref for Properties {
    type Target = BTreeMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
