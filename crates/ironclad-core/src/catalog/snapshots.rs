use std::collections::HashMap;

use hex::ToHex;
use sha2::{Digest, Sha256};

use crate::{
    catalog::{Catalog, error::CatalogError},
    fact::LabeledFact,
    recipe::RecipeProgressEvent,
    sample::{Sample, Trace, batch::Batch},
    snapshot::Snapshot,
};

pub enum SnapshotProgressEvent<'a> {
    FactStarted {
        index: usize,
        fact: &'a LabeledFact,
    },
    FactFinished {
        index: usize,
        fact: &'a LabeledFact,
        output: &'a Vec<Sample>,
    },
    FactStep {
        index: usize,
        fact: &'a LabeledFact,
        inner: RecipeProgressEvent<'a>,
    },
}

impl Catalog {
    pub fn capture_snapshot<F: FnMut(SnapshotProgressEvent)>(
        &self,
        facts: Vec<LabeledFact>,
        redact_secrets: bool,
        mut on_progress: F,
    ) -> Result<Snapshot, CatalogError> {
        let snapshot = Snapshot::new(HashMap::from_iter(
            facts
                .into_iter()
                .zip(0..)
                .try_fold(
                    (Vec::new(), HashMap::new()),
                    |(mut snapshot_entries, mut exported_samples),
                     (fact, index)|
                     -> Result<_, CatalogError> {
                        on_progress(SnapshotProgressEvent::FactStarted { index, fact: &fact });

                        let imports = fact
                            .imports()
                            .iter()
                            .map(|key| {
                                exported_samples
                                    .get(key)
                                    .ok_or_else(|| CatalogError::ImportNotFound(key.clone()))
                                    .map(|sample| (key, sample))
                            })
                            .collect::<Result<HashMap<_, _>, _>>()?;

                        let samples = fact.steps().eval(self, &imports, |update| {
                            on_progress(SnapshotProgressEvent::FactStep {
                                index,
                                fact: &fact,
                                inner: update,
                            })
                        })?;

                        on_progress(SnapshotProgressEvent::FactFinished {
                            index,
                            fact: &fact,
                            output: &samples,
                        });

                        let batch = Batch::new(if fact.secret() && redact_secrets {
                            samples.into_iter().map(redact_sample).collect()
                        } else {
                            samples
                        });

                        for (key, entry) in fact.fact.into_exports() {
                            let value = batch
                                .samples()
                                .iter()
                                .find(|sample| {
                                    sample.traces().iter().any(|trace| {
                                        trace.entries().get_key_value(&entry.trace_key)
                                            == Some((&entry.trace_key, &entry.trace_value))
                                    })
                                })
                                .ok_or_else(|| CatalogError::SampleToExportNotFound {
                                    fact_label: fact.label.clone(),
                                    export_key: key.clone(),
                                    trace_key: entry.trace_key,
                                    trace_value: entry.trace_value,
                                })?;
                            exported_samples.insert(key, value.clone());
                        }

                        snapshot_entries.push((fact.label, batch));

                        Ok((snapshot_entries, exported_samples))
                    },
                )?
                .0,
        ));
        Ok(snapshot)
    }
}

fn redact_sample(sample: Sample) -> Sample {
    let digest: String = Sha256::digest(sample.content()).encode_hex();
    sample.evolve(
        Trace::new(HashMap::from([("digest".to_string(), digest.clone())])),
        format!("[redacted secret: {digest}]"),
    )
}
