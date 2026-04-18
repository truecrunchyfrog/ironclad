use std::collections::HashMap;

use hex::ToHex;
use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator};
use sha2::{Digest, Sha256};

use crate::{
    catalog::{Catalog, error::CatalogError},
    sample::{Trace, batch::Batch},
    snapshot::Snapshot,
};

impl Catalog {
    pub fn capture_snapshot(&self, redact_secrets: bool) -> Result<Snapshot, CatalogError> {
        // TODO facts as argument instead
        let facts = self.load_facts()?;

        Ok(Snapshot::new(HashMap::from_par_iter(
            facts
                .into_iter()
                .map(|(_label, fact_id, _path, fact)| {
                    let samples = fact.steps().eval(self)?;
                    let batch = Batch::new(if fact.secret() && redact_secrets {
                        samples
                            .into_iter()
                            .map(|sample| {
                                let digest: String = Sha256::digest(sample.content()).encode_hex();
                                sample.evolve(
                                    Trace::new(HashMap::from([(
                                        "digest".to_string(),
                                        digest.clone(),
                                    )])),
                                    format!("[redacted secret: {}]", digest),
                                )
                            })
                            .collect()
                    } else {
                        samples
                    });
                    Ok((fact_id, batch))
                })
                .collect::<Result<Vec<_>, CatalogError>>()?
                // TODO to par or not to par?
                .into_par_iter()
                .map(|(fact_id, batch)| Ok((fact_id, batch)))
                .collect::<Result<Vec<_>, CatalogError>>()?,
        )))
    }
}
