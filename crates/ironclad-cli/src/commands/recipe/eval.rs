use anyhow::anyhow;
use ironclad_core::{fact::id::FactId, sample::Sample};

use crate::{args::recipe::eval::EvalRecipeArgs, config::Config, helper::resolve_catalog, ui};

pub(super) fn dispatch(_config: &Config, args: EvalRecipeArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;
    let fact = catalog.load_fact_for_id(&FactId::from(args.fact_id))?;

    let step_len = fact.recipe().steps().len();

    let eval_steps = fact
        .recipe()
        .steps()
        .iter()
        .zip(0..)
        .filter(|(_, index)| {
            args.indices
                .as_ref()
                .is_none_or(|indices| indices.contains(index))
        })
        .collect::<Vec<_>>();

    if eval_steps.is_empty() {
        return Err(anyhow!("empty recipe"));
    }

    eval_steps.into_iter().try_fold(
        Vec::new(),
        |input, (step, index)| -> anyhow::Result<Vec<Sample>> {
            ui::info(format!(
                "{}  {}  {}",
                index,
                step.operation_id(),
                step.options()
            ));

            let output = step.eval(&catalog, input)?;

            if args.show_all
                || args
                    .show
                    .as_ref()
                    .is_some_and(|indices| indices.contains(&index))
                || (args.show.is_none() && index == step_len - 1)
            {
                println!("{}", serde_json::to_string_pretty(&output)?);
            }

            Ok(output)
        },
    )?;

    Ok(())
}
