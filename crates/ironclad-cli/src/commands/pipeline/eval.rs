use anyhow::anyhow;
use ironclad_core::sample::Sample;

use crate::{
    args::pipeline::eval::EvalPipelineArgs,
    config::Config,
    helper::{resolve_explicit_or_reused_cell, resolve_ledger},
    ui,
};

pub(super) fn dispatch(_config: &Config, args: EvalPipelineArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;
    let cell = resolve_explicit_or_reused_cell(&ledger, args.cell_id)?;

    let stage_len = cell.pipeline().stages().len();

    let eval_stages = cell
        .pipeline()
        .stages()
        .into_iter()
        .zip(0..)
        .filter(|(_, index)| {
            args.indices
                .as_ref()
                .is_none_or(|indices| indices.contains(index))
        })
        .collect::<Vec<_>>();

    if eval_stages.is_empty() {
        return Err(anyhow!("empty pipeline"));
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

            let output = stage.eval(&ledger, input)?;

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
