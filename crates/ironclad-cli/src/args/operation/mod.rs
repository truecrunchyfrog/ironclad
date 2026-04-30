pub(crate) mod eval;
pub(crate) mod list;
pub(crate) mod show;

use clap::Subcommand;

use crate::args::operation::{
    eval::EvalOperationArgs, list::ListOperationArgs, show::ShowOperationArgs,
};

/// Inspect and evaluate operations.
///
/// The `op` command family is for operation-level work: listing available
/// operations, showing one operation in detail, and evaluating an operation
/// without writing a fact.
///
/// This is useful when you are designing a pipeline or debugging one step at a
/// time.
#[derive(Subcommand)]
pub(crate) enum OperationCommand {
    Eval(EvalOperationArgs),
    #[command(alias = "ls")]
    List(ListOperationArgs),
    #[command(alias = "sh")]
    Show(ShowOperationArgs),
}
