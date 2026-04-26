use ironclad_core::{catalog::Catalog, fact::Fact};

pub(crate) fn resolve_catalog() -> anyhow::Result<Catalog> {
    Ok(Catalog::find_for_working_dir(&std::env::current_dir()?)?)
}

pub(crate) struct ResolvedFact {
    pub(crate) selector: String,
    pub(crate) fact_id: String,
    pub(crate) fact: Fact,
}

pub(crate) fn resolve_fact(catalog: &Catalog, selector: &str) -> anyhow::Result<ResolvedFact> {
    let fact_id = catalog.resolve_fact_id(selector)?;
    let fact = catalog.load_fact_for_path(&catalog.fact_file_path(&fact_id))?;

    Ok(ResolvedFact {
        selector: selector.to_string(),
        fact_id,
        fact,
    })
}
