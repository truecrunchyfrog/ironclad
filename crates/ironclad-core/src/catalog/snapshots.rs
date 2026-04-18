use std::collections::HashMap;

use rayon::iter::{FromParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    catalog::{Catalog, error::CatalogError},
    snapshot::Snapshot,
};

impl Catalog {
    pub fn capture_snapshot(&self) -> Result<Snapshot, CatalogError> {
        let facts = self.load_facts()?;

        let batches = facts
            .into_iter()
            .map(|(_label, fact_id, _path, fact)| Ok((fact_id.clone(), fact.recipe().eval(self)?)))
            .collect::<Result<Vec<_>, CatalogError>>()?;

        Ok(Snapshot::new(HashMap::from_par_iter(
            batches
                // TODO to par or not to par?
                .par_iter()
                .map(|(fact_id, batch)| Ok((fact_id.clone(), batch.to_owned())))
                .collect::<Result<Vec<_>, CatalogError>>()?,
        )))
    }
}
