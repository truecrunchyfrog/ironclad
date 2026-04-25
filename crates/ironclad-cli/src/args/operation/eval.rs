use clap::Args;
use clap_stdin::MaybeStdin;

/// Evaluate an operation.
#[derive(Args)]
pub(crate) struct EvalOperationArgs {
    /// ID of operation to evaluate.
    pub(crate) operation_id: String,

    /// File with batch to send to operation, instead of passing an empty batch. '-' for stdin.
    #[arg(short, long)]
    pub(crate) input: Option<MaybeStdin<String>>,

    /// Options to pass to the operation, in TOML.
    #[arg(short, long)]
    pub(crate) options: Option<MaybeStdin<toml::Value>>,
}
