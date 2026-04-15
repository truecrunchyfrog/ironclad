use ironclad_core::catalog::Catalog;

pub(crate) fn resolve_catalog() -> anyhow::Result<Catalog> {
    Ok(Catalog::find_for_working_dir(&std::env::current_dir()?)?)
}
