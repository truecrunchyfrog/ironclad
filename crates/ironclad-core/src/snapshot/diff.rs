use std::collections::{HashMap, HashSet};

use serde::Serialize;

use crate::{
    sample::{Sample, batch::Batch},
    snapshot::Snapshot,
};

#[derive(Debug, Clone, Serialize)]
pub struct BatchDiff {
    before: Option<Batch>,
    after: Option<Batch>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchStatus {
    New,
    Removed,
    Changed,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleChangeKind {
    Removed,
    Added,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeCounts {
    pub removed: usize,
    pub added: usize,
}

pub struct SampleChange<'a> {
    kind: SampleChangeKind,
    before: Option<&'a Sample>,
    after: Option<&'a Sample>,
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
        self.before.as_ref().map(Batch::samples) == self.after.as_ref().map(Batch::samples)
    }

    #[must_use]
    pub fn status(&self) -> BatchStatus {
        match (self.before(), self.after()) {
            (None, Some(_)) => BatchStatus::New,
            (Some(_), None) => BatchStatus::Removed,
            (Some(_), Some(_)) if self.batches_equal() => BatchStatus::Unchanged,
            (Some(_), Some(_)) => BatchStatus::Changed,
            (None, None) => BatchStatus::Unchanged,
        }
    }

    #[must_use]
    pub fn change_counts(&self) -> ChangeCounts {
        self.sample_changes().into_iter().fold(
            ChangeCounts {
                removed: 0,
                added: 0,
            },
            |mut counts, change| {
                match change.kind() {
                    SampleChangeKind::Removed => counts.removed += 1,
                    SampleChangeKind::Added => counts.added += 1,
                    SampleChangeKind::Unchanged => {}
                }
                counts
            },
        )
    }

    #[must_use]
    pub fn sample_changes(&self) -> Vec<SampleChange<'_>> {
        let samples_before = self.before.as_ref().map(Batch::samples).unwrap_or_default();
        let samples_after = self.after.as_ref().map(Batch::samples).unwrap_or_default();

        let mut result = Vec::new();
        let mut matched_after = vec![false; samples_after.len()];

        for sample in samples_before {
            if let Some(index) = samples_after
                .iter()
                .enumerate()
                .find_map(|(index, candidate)| {
                    (!matched_after[index] && candidate == sample).then_some(index)
                })
            {
                matched_after[index] = true;
                result.push(SampleChange {
                    kind: SampleChangeKind::Unchanged,
                    before: Some(sample),
                    after: Some(&samples_after[index]),
                });
            } else {
                result.push(SampleChange {
                    kind: SampleChangeKind::Removed,
                    before: Some(sample),
                    after: None,
                });
            }
        }

        for (index, sample) in samples_after.iter().enumerate() {
            if !matched_after[index] {
                result.push(SampleChange {
                    kind: SampleChangeKind::Added,
                    before: None,
                    after: Some(sample),
                });
            }
        }

        result
    }
}

impl<'a> SampleChange<'a> {
    #[must_use]
    pub fn kind(&self) -> SampleChangeKind {
        self.kind
    }

    #[must_use]
    pub fn before(&self) -> Option<&'a Sample> {
        self.before
    }

    #[must_use]
    pub fn after(&self) -> Option<&'a Sample> {
        self.after
    }
}

impl Snapshot {
    #[must_use]
    pub fn diff(&self, before: &Self) -> HashMap<String, BatchDiff> {
        let before = before.entries();
        let after = self.entries();

        let fact_ids = HashSet::<String>::from_iter(before.keys().chain(after.keys()).cloned());

        HashMap::from_iter(fact_ids.into_iter().map(|fact_id| {
            let before = before.get(&fact_id);
            let after = after.get(&fact_id);

            (
                fact_id,
                BatchDiff {
                    before: before.cloned(),
                    after: after.cloned(),
                },
            )
        }))
    }

    #[must_use]
    pub fn sorted_diff<'a>(&'a self, before: &'a Self) -> Vec<(&'a String, BatchDiff)> {
        let mut diff = self.diff(before).into_iter().collect::<Vec<_>>();
        diff.sort_by(|a, b| a.0.cmp(&b.0));
        diff.into_iter()
            .map(|(label, diff)| {
                let label_ref = self
                    .entries()
                    .get_key_value(&label)
                    .map(|(key, _)| key)
                    .or_else(|| before.entries().get_key_value(&label).map(|(key, _)| key))
                    .expect("diff label should exist in one snapshot");
                (label_ref, diff)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        sample::{Sample, Trace, batch::Batch},
        snapshot::{
            Snapshot,
            diff::{BatchStatus, SampleChangeKind},
        },
    };

    fn sample(content: &str) -> Sample {
        Sample::new(Trace::new(HashMap::new()), content.to_string())
    }

    #[test]
    fn multiplicity_changes_are_not_equal() {
        let before = Snapshot::new(HashMap::from([(
            "fact".to_string(),
            Batch::new(vec![sample("same")]),
        )]));
        let after = Snapshot::new(HashMap::from([(
            "fact".to_string(),
            Batch::new(vec![sample("same"), sample("same")]),
        )]));

        let diff = after.diff(&before);

        assert!(!diff["fact"].batches_equal());
    }

    #[test]
    fn duplicate_samples_are_reported_in_diffs() {
        let before = Snapshot::new(HashMap::from([(
            "fact".to_string(),
            Batch::new(vec![sample("same")]),
        )]));
        let after = Snapshot::new(HashMap::from([(
            "fact".to_string(),
            Batch::new(vec![sample("same"), sample("same")]),
        )]));

        let diff = after.diff(&before);
        let sample_diffs = diff["fact"].sample_changes();

        assert_eq!(sample_diffs.len(), 2);
        assert_eq!(sample_diffs[0].kind(), SampleChangeKind::Unchanged);
        assert_eq!(sample_diffs[1].kind(), SampleChangeKind::Added);
    }

    #[test]
    fn reports_batch_status_and_change_counts() {
        let before = Snapshot::new(HashMap::from([(
            "fact".to_string(),
            Batch::new(vec![sample("alpha"), sample("beta")]),
        )]));
        let after = Snapshot::new(HashMap::from([(
            "fact".to_string(),
            Batch::new(vec![sample("alpha"), sample("gamma")]),
        )]));

        let diff = after.diff(&before);
        let batch_diff = &diff["fact"];

        assert_eq!(batch_diff.status(), BatchStatus::Changed);
        assert_eq!(batch_diff.change_counts().removed, 1);
        assert_eq!(batch_diff.change_counts().added, 1);
    }
}
