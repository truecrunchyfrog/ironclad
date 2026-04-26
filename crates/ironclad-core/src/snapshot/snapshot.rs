use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sample::batch::Batch;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Snapshot(HashMap<String, Batch>);

impl Snapshot {
    #[must_use]
    pub fn new(entries: HashMap<String, Batch>) -> Self {
        Self(entries)
    }

    #[must_use]
    pub fn into_entries(self) -> HashMap<String, Batch> {
        self.0
    }

    #[must_use]
    pub fn entries(&self) -> &HashMap<String, Batch> {
        &self.0
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<String, Batch> {
        &mut self.0
    }

    #[must_use]
    pub fn batch(&self, label: &str) -> Option<&Batch> {
        self.0.get(label)
    }

    #[must_use]
    pub fn into_batch(mut self, label: &str) -> Option<Batch> {
        self.0.remove(label)
    }

    #[must_use]
    pub fn sorted_entries(&self) -> Vec<(&String, &Batch)> {
        let mut entries = self.0.iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries
    }

    #[must_use]
    pub fn into_sorted_entries(self) -> Vec<(String, Batch)> {
        let mut entries = self.0.into_iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        entries
    }
}
