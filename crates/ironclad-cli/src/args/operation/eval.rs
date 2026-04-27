use clap::Args;
use clap_stdin::MaybeStdin;

/// Evaluate one operation by hand.
#[derive(Args)]
#[command(long_about = "Evaluate an operation.\n\n\
`op eval` runs one operation against a JSON batch of samples and prints the \
resulting batch as JSON.\n\n\
This command is useful when you want to prototype a step before adding it to a \
fact. Operations that do not require a catalog can be evaluated outside a \
catalog directory.\n\n\
Avoid reading both `--input` and `--options` from stdin in the same invocation.")]
pub(crate) struct EvalOperationArgs {
    /// Operation ID to evaluate.
    pub(crate) operation_id: String,

    /// Input batch as JSON. Use `-` for stdin.
    #[arg(short, long)]
    pub(crate) input: Option<MaybeStdin<String>>,

    /// Operation options as TOML. Use `-` for stdin.
    #[arg(short, long)]
    pub(crate) options: Option<MaybeStdin<toml::Value>>,
}
