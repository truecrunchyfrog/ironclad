use clap::Args;

/// List registered operations.
///
/// `op list` prints the operation IDs available in the current build, one per
/// line.
///
/// Use `op show <operation-id>` for details about one operation.
#[derive(Args)]
pub(crate) struct ListOperationArgs {}
