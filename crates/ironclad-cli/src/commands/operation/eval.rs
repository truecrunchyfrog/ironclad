use ironclad_core::{registry, sample::Sample};

use crate::{args::operation::eval::EvalOperationArgs, config::Config, helper::resolve_catalog};

pub(super) fn dispatch(_config: &Config, args: EvalOperationArgs) -> anyhow::Result<()> {
    let catalog = resolve_catalog()?;

    let options = match args.options {
        Some(serialized) => serde_json::from_str::<serde_json::Value>(&serialized)?,
        None => serde_json::Value::Null,
    };

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
