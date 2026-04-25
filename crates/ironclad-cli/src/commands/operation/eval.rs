use clap_stdin::MaybeStdin;
use ironclad_core::{registry, sample::Sample};

use crate::{args::operation::eval::EvalOperationArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: EvalOperationArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let options = args.options.map(MaybeStdin::into_inner);

    let operation = registry::resolve_op(&args.operation_id)?;

    let input = args
        .input
        .map(|input| serde_json::from_str::<Vec<Sample>>(&input))
        .transpose()?
        .unwrap_or_default();

    let output = operation.eval(&catalog, input, options)?;

    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}
