use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

/// Contains all tags for telemetry to submit.
#[derive(Clone, Default)]
pub struct ContextTags(BTreeMap<String, String>);

impl ContextTags {
    // Combines all tags from two bags. It can override some tags with values found
    // in the second tags bag.
    pub fn combine(a: ContextTags, b: ContextTags) -> Self {
        let items = a.0.into_iter().chain(b.0).collect();
        Self(items)
    }
}

impl From<ContextTags> for BTreeMap<String, String> {
    fn from(tags: ContextTags) -> Self {
        tags.0
    }
}

impl Deref for ContextTags {
    type Target = BTreeMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ContextTags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
