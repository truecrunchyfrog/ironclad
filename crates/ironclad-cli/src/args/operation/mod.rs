pub(crate) mod eval;
pub(crate) mod list;
pub(crate) mod show;

use clap::Subcommand;

use crate::args::operation::{
    eval::EvalOperationArgs, list::ListOperationArgs, show::ShowOperationArgs,
};

/// Inspect and evaluate operations.
#[derive(Subcommand)]
pub(crate) enum OperationCommand {
    #[command(
        about = "Evaluate one operation by hand.",
        long_about = "Evaluate one operation by hand.\n\n\
`op eval` runs a single operation without creating a fact. It is intended for \
pipeline prototyping and debugging.\n\n\
Pass an input batch as JSON, pass operation options as TOML, and inspect the \
resulting batch on stdout."
    )]
    Eval(EvalOperationArgs),
    #[command(alias = "ls")]
    #[command(
        about = "List registered operation IDs.",
        long_about = "List registered operation IDs.\n\n\
`op list` prints one operation ID per line. Use it to discover the operation \
names available in the current build."
    )]
    List(ListOperationArgs),
    #[command(alias = "sh")]
    #[command(
        about = "Show one operation in detail.",
        long_about = "Show one operation in detail.\n\n\
`op show` prints an operation's ID, description, and default options template. \
Use it when you want to understand one operation before using it in a fact or \
with `op eval`."
    )]
    Show(ShowOperationArgs),
}
