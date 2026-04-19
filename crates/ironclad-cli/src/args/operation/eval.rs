use clap::Args;

/// Evaluate an operation.
#[derive(Args)]
pub(crate) struct EvalOperationArgs {
    /// ID of operation to evaluate.
    pub(crate) operation_id: String,

    /// Feed operation with an empty batch instead of reading from stdin.
    #[arg(short = 'h', long)]
    pub(crate) head: bool,

    /// Options to pass to the operation, in JSON.
    #[arg(short, long)]
    pub(crate) options: Option<String>,
}
