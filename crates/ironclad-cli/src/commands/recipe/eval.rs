use anyhow::anyhow;
use ironclad_core::sample::Sample;

use crate::{
    args::recipe::eval::EvalRecipeArgs,
    config::Config,
    helper::{resolve_catalog, resolve_explicit_or_reused_fact},
    ui,
};

pub(super) fn dispatch(_config: &Config, args: EvalRecipeArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let fact = resolve_explicit_or_reused_fact(&catalog, args.fact_id)?;

    let stage_len = fact.recipe().stages().len();

    let eval_stages = fact
        .recipe()
        .stages()
        .iter()
        .zip(0..)
        .filter(|(_, index)| {
            args.indices
                .as_ref()
                .is_none_or(|indices| indices.contains(index))
        })
        .collect::<Vec<_>>();

    if eval_stages.is_empty() {
        return Err(anyhow!("empty recipe"));
    }

    eval_stages.into_iter().try_fold(
        Vec::new(),
        |input, (stage, index)| -> anyhow::Result<Vec<Vec<Sample>>> {
            ui::info(format!(
                "{}  {}  {}",
                index,
                stage.operation_id(),
                stage.options()
            ));

            let output = stage.eval(&catalog, input)?;

            if args.show_all
                || args
                    .show
                    .as_ref()
                    .is_some_and(|indices| indices.contains(&index))
                || (args.show.is_none() && index == stage_len - 1)
            {
                println!("{}", serde_json::to_string_pretty(&output)?);
            }

            Ok(output)
        },
    )?;

    Ok(())
}
