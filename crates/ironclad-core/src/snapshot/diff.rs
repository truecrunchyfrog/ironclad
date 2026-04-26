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
        self.before.as_ref().map(Batch::samples) == self.after.as_ref().map(Batch::samples)
    }

    #[must_use]
    pub fn sample_diffs(&self) -> Vec<(&Sample, SamplePresence)> {
        let samples_before = self.before.as_ref().map(Batch::samples).unwrap_or_default();
        let samples_after = self.after.as_ref().map(Batch::samples).unwrap_or_default();

        let mut result = Vec::new();
        let mut matched_after = vec![false; samples_after.len()];

        for sample in samples_before {
            let presence = if let Some(index) =
                samples_after
                    .iter()
                    .enumerate()
                    .find_map(|(index, candidate)| {
                        (!matched_after[index] && candidate == sample).then_some(index)
                    }) {
                matched_after[index] = true;
                SamplePresence::Both
            } else {
                SamplePresence::OnlyBefore
            };

            result.push((sample, presence));
        }

        for (index, sample) in samples_after.iter().enumerate() {
            if !matched_after[index] {
                result.push((sample, SamplePresence::OnlyAfter));
            }
        }

        result
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
        snapshot::Snapshot,
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
        let sample_diffs = diff["fact"].sample_diffs();

        assert_eq!(sample_diffs.len(), 2);
        assert_eq!(sample_diffs[0].1, super::SamplePresence::Both);
        assert_eq!(sample_diffs[1].1, super::SamplePresence::OnlyAfter);
    }
}
