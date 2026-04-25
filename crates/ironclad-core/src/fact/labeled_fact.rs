use std::{hash::Hash, ops::Deref};

use crate::fact::Fact;

pub struct LabeledFact {
    pub label: String,
    pub fact: Fact,
}

impl Deref for LabeledFact {
    type Target = Fact;

    fn deref(&self) -> &Self::Target {
        &self.fact
    }
}

impl PartialEq for LabeledFact {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for LabeledFact {}

impl Hash for LabeledFact {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}
