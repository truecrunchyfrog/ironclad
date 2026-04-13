use anyhow::anyhow;
use ironclad_core::fact::id::FactId;

use crate::{args::recipe::pop::PopRecipeArgs, config::Config, helper::resolve_catalog, ui};

pub(super) fn dispatch(_config: &Config, args: PopRecipeArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let mut fact = catalog.load_fact_for_id(&FactId::from(args.fact_id))?;

    if fact.recipe().steps().is_empty() {
        return Err(anyhow!("empty recipe"));
    }

    let removed_step = fact.recipe_mut().remove(args.index)?;

    catalog.save_fact(&fact)?;

    ui::info(format!(
        "removed operation '{}' with options '{}'",
        removed_step.operation_id(),
        removed_step.options()
    ));

    Ok(())
}
