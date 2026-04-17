use anyhow::anyhow;
use ironclad_core::catalog::Catalog;

use crate::{args::recipe::pop::PopRecipeArgs, config::Config, helper::resolve_catalog, ui};

pub(super) fn dispatch(_config: &Config, args: PopRecipeArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let index = catalog.load_fact_index()?;
    let fact_id = Catalog::fact_id_for_label(&index, &args.label)?;
    let path = catalog.fact_file_path(&fact_id);
    let mut fact = catalog.load_fact_for_path(&path)?;

    if fact.recipe().steps().is_empty() {
        return Err(anyhow!("empty recipe"));
    }

    let removed_step = fact.recipe_mut().remove(args.index)?;

    std::fs::write(path, serde_json::to_vec_pretty(&fact)?)?;

    ui::info(format!(
        "removed operation '{}' with options '{}'",
        removed_step.operation_id(),
        removed_step.options()
    ));

    Ok(())
}
