use std::collections::{HashMap, HashSet};

use crate::{
    fact::id::FactId,
    sample::{Sample, batch::Batch},
    snapshot::Snapshot,
};

#[derive(Debug, Clone)]
pub struct BatchDiff {
    before: Option<Batch>,
    after: Option<Batch>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SamplePresence {
    OnlyBefore,
    OnlyAfter,
    Both,
}

impl BatchDiff {
    #[must_use]
    pub fn before(&self) -> &Option<Batch> {
        &self.before
    }

    #[must_use]
    pub fn after(&self) -> &Option<Batch> {
        &self.after
    }

    #[must_use]
    pub fn batches_equal(&self) -> bool {
        self.before
            .as_ref()
            .map(|batch| batch.samples().iter().collect::<HashSet<_>>())
            == self
                .after
                .as_ref()
                .map(|batch| batch.samples().iter().collect::<HashSet<_>>())
    }

    #[must_use]
    pub fn sample_diffs(&self) -> Vec<(&Sample, SamplePresence)> {
        let samples_before = self
            .before
            .as_ref()
            .map(|batch| batch.samples())
            .unwrap_or_default();
        let samples_after = self
            .after
            .as_ref()
            .map(|batch| batch.samples())
            .unwrap_or_default();

        let mut result = Vec::new();

        for sample in samples_before {
            let presence = if samples_after.contains(&sample) {
                SamplePresence::Both
            } else {
                SamplePresence::OnlyBefore
            };

            result.push((sample, presence));
        }

        for sample in samples_after {
            if !result.iter().any(|(s, _)| s == &sample) {
                result.push((sample, SamplePresence::OnlyAfter));
            }
        }

        result
    }
}

impl Snapshot {
    #[must_use]
    pub fn diff(&self, before: &Self) -> HashMap<FactId, BatchDiff> {
        let before = before.entries();
        let after = self.entries();

        let fact_ids = HashSet::<FactId>::from_iter(before.keys().chain(after.keys()).cloned());

        HashMap::from_iter(fact_ids.into_iter().map(|fact_id| {
            let before = before.get(&fact_id);
            let after = after.get(&fact_id);

            (
                fact_id,
                BatchDiff {
                    before: before.map(|e| e.clone()),
                    after: after.map(|e| e.clone()),
                },
            )
        }))
    }
}
