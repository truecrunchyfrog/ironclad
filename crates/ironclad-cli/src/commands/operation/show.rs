use crate::{args::operation::show::ShowOperationArgs, context::Context};

pub(super) fn dispatch(context: &Context, args: ShowOperationArgs) -> anyhow::Result<()> {
    let operation = context.registry().resolve_op(&args.operation_id)?;

    println!("{}", args.operation_id);
    println!();
    println!("{}", operation.description());

    Ok(())
}
