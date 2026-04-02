use std::collections::{HashMap, HashSet};

use crate::{
    cell::id::CellId,
    sample::{Sample, batch::Batch},
    snapshot::Snapshot,
};

#[derive(Debug, Clone)]
pub struct BatchDiff {
    before: Option<Batch>,
    after: Option<Batch>,
}

#[derive(Debug, PartialEq)]
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
    pub fn sample_diffs(self) -> Vec<(Sample, SamplePresence)> {
        let samples_before = self
            .before
            .map_or(Vec::new(), super::super::sample::batch::Batch::into_samples);
        let samples_after = self
            .after
            .map_or(Vec::new(), super::super::sample::batch::Batch::into_samples);

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
    pub fn diff(&self, before: Self) -> HashMap<CellId, (BatchDiff, Vec<(CellId, BatchDiff)>)> {
        let before = before.entries();
        let after = self.entries();

        let cell_ids = HashSet::<CellId>::from_iter(before.keys().chain(after.keys()).cloned());

        HashMap::from_iter(cell_ids.into_iter().map(|cell_id| {
            let before = before.get(&cell_id);
            let after = after.get(&cell_id);

            let dep_cell_ids = HashSet::<CellId>::from_iter(
                before
                    .iter()
                    .chain(after.iter())
                    .flat_map(|e| e.dependencies().keys())
                    .cloned(),
            );

            (
                cell_id,
                (
                    BatchDiff {
                        before: before.map(|e| e.batch().clone()),
                        after: after.map(|e| e.batch().clone()),
                    },
                    dep_cell_ids
                        .into_iter()
                        .map(|dep_cell_id| {
                            (
                                dep_cell_id.clone(),
                                BatchDiff {
                                    before: before
                                        .and_then(|e| e.dependencies().get(&dep_cell_id))
                                        .cloned(),
                                    after: after
                                        .and_then(|e| e.dependencies().get(&dep_cell_id))
                                        .cloned(),
                                },
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            )
        }))
    }
}
