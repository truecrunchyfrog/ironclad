use std::collections::HashMap;

use hex::ToHex;
use sha2::{Digest, Sha256};

use crate::{
    catalog::{Catalog, error::CatalogError},
    fact::{LabeledFact, RecipeProgressEvent, dependencies::sort_dependencies},
    registry::Registry,
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
        registry: &Registry,
        facts: Vec<LabeledFact>,
        redact_secrets: bool,
        mut on_progress: F,
    ) -> Result<Snapshot, CatalogError> {
        let facts = sort_dependencies(facts)?;
        validate_unique_export_keys(&facts)?;

        let snapshot = Snapshot::new(HashMap::from_iter(
            facts
                .into_iter()
                .enumerate()
                .try_fold(
                    (Vec::new(), HashMap::new()),
                    |(mut snapshot_entries, mut exported_samples),
                     (index, fact)|
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

                        let samples = fact.eval(registry, self, &imports, |update| {
                            on_progress(SnapshotProgressEvent::FactStep {
                                index,
                                fact: &fact,
                                inner: update,
                            });
                        })?;

                        on_progress(SnapshotProgressEvent::FactFinished {
                            index,
                            fact: &fact,
                            output: &samples,
                        });

                        let should_redact = fact.secret() && redact_secrets;

                        for (key, entry) in fact.fact.into_exports() {
                            let value = samples
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

                        let batch = Batch::new(if should_redact {
                            samples.into_iter().map(redact_sample).collect()
                        } else {
                            samples
                        });

                        snapshot_entries.push((fact.label, batch));

                        Ok((snapshot_entries, exported_samples))
                    },
                )?
                .0,
        ));
        Ok(snapshot)
    }
}

fn validate_unique_export_keys(facts: &[LabeledFact]) -> Result<(), CatalogError> {
    let mut fact_labels_by_export_key = HashMap::<String, Vec<String>>::new();

    for fact in facts {
        for key in fact.exports().keys() {
            fact_labels_by_export_key
                .entry(key.clone())
                .or_default()
                .push(fact.label.clone());
        }
    }

    if let Some((key, fact_labels)) = fact_labels_by_export_key
        .into_iter()
        .find(|(_, fact_labels)| fact_labels.len() > 1)
    {
        return Err(CatalogError::DuplicateExportKey { key, fact_labels });
    }

    Ok(())
}

fn redact_sample(sample: Sample) -> Sample {
    let digest: String = Sha256::digest(sample.content()).encode_hex();
    sample.evolve(
        Trace::new(HashMap::from([("digest".to_string(), digest.clone())])),
        format!("[redacted secret: {digest}]"),
    )
}
