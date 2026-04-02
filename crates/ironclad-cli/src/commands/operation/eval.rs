use std::io::{Read, stdin};

use ironclad_core::{registry, sample::Sample};

use crate::{args::operation::eval::EvalOperationArgs, config::Config, helper::resolve_ledger};

pub(super) fn dispatch(_config: &Config, args: EvalOperationArgs) -> anyhow::Result<()> {
    let ledger = resolve_ledger()?;

    let options = match args.options {
        Some(serialized) => serde_json::from_str::<serde_json::Value>(&serialized)?,
        None => serde_json::Value::Null,
    };

    let operation = registry::resolve_op(&args.operation_id)?;

    let input = if args.head {
        Vec::new()
    } else {
        let mut buf = Vec::new();
        stdin().read_to_end(&mut buf)?;
        serde_json::from_slice::<Vec<Vec<Sample>>>(buf.as_slice())?
    };

    let output = operation.eval(&ledger, input, options)?;

    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}
