use clap::Args;

/// List registered operations.
#[derive(Args)]
#[command(long_about = "List registered operations.\n\n\
`op list` prints the operation IDs available in the current build, one per \
line.\n\n\
Use `op show <operation-id>` for details about one operation.")]
pub(crate) struct ListOperationArgs {}
