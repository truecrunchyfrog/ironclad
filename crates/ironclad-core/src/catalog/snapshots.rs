use std::collections::HashMap;

use hex::ToHex;
use sha2::{Digest, Sha256};

use crate::{
    catalog::{Catalog, error::CatalogError},
    fact::Fact,
    recipe::RecipeProgressEvent,
    sample::{Sample, Trace, batch::Batch},
    snapshot::Snapshot,
};

pub enum SnapshotProgressEvent<'a> {
    BeforeEvaluateFact {
        label: &'a str,
        fact: &'a Fact,
    },
    AfterEvaluateFact {
        label: &'a str,
        fact: &'a Fact,
        output: &'a Vec<Sample>,
    },
    Recipe(RecipeProgressEvent<'a>),
}

impl Catalog {
    pub fn capture_snapshot<F: FnMut(SnapshotProgressEvent)>(
        &self,
        facts: Vec<(String, Fact)>,
        redact_secrets: bool,
        mut on_progress: F,
    ) -> Result<Snapshot, CatalogError> {
        let snapshot = Snapshot::new(HashMap::from_iter(
            facts
                .into_iter()
                .map(|(label, fact)| {
                    on_progress(SnapshotProgressEvent::BeforeEvaluateFact {
                        label: &label,
                        fact: &fact,
                    });
                    let samples = fact.steps().eval(self, |update| {
                        on_progress(SnapshotProgressEvent::Recipe(update))
                    })?;
                    on_progress(SnapshotProgressEvent::AfterEvaluateFact {
                        label: &label,
                        fact: &fact,
                        output: &samples,
                    });
                    let batch = Batch::new(if fact.secret() && redact_secrets {
                        samples.into_iter().map(redact_sample).collect()
                    } else {
                        samples
                    });
                    Ok((label, batch))
                })
                .collect::<Result<Vec<_>, CatalogError>>()?,
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
