use std::collections::HashMap;

use hex::ToHex;
use sha2::{Digest, Sha256};

use crate::{
    catalog::{Catalog, error::CatalogError},
    fact::Fact,
    sample::{Sample, Trace, batch::Batch},
    snapshot::Snapshot,
};

impl Catalog {
    pub fn capture_snapshot(
        &self,
        facts: Vec<(String, Fact)>,
        redact_secrets: bool,
    ) -> Result<Snapshot, CatalogError> {
        let snapshot = Snapshot::new(HashMap::from_iter(
            facts
                .into_iter()
                .map(|(label, fact)| {
                    let samples = fact.steps().eval(self)?;
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
