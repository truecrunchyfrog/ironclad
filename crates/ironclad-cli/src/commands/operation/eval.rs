use clap_stdin::MaybeStdin;
use ironclad_core::sample::Sample;

use crate::{args::operation::eval::EvalOperationArgs, context::Context};

pub(super) fn dispatch(context: &Context, args: EvalOperationArgs) -> anyhow::Result<()> {
    let session = context.catalog_session()?;

    let options = args.options.map(MaybeStdin::into_inner);

    let operation = context.registry().resolve_op(&args.operation_id)?;

    let input = args
        .input
        .map(|input| serde_json::from_str::<Vec<Sample>>(&input))
        .transpose()?
        .unwrap_or_default();

    let output = operation.eval(session.catalog(), input, options)?;

    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}
