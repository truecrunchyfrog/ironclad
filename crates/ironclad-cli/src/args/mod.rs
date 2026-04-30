pub(crate) mod apply;
pub(crate) mod catalog;
pub(crate) mod check;
pub(crate) mod diff;
pub(crate) mod fact;
pub(crate) mod inspect;
pub(crate) mod operation;
pub(crate) mod resolve;

use clap::{Parser, Subcommand};

use crate::{
    args::{
        apply::ApplyArgs,
        catalog::init::InitCatalogArgs,
        check::CheckArgs,
        diff::DiffArgs,
        fact::{
            add::AddFactArgs, edit::EditFactArgs, list::ListFactArgs, remove::RemoveFactArgs,
            rename::RenameFactArgs, show::ShowFactArgs,
        },
        inspect::InspectArgs,
        operation::OperationCommand,
        resolve::ResolveArgs,
    },
    config::Config,
};

/// Track and review small pieces of external state.
///
/// Ironclad stores facts in a catalog, resolves them into snapshots, compares the
/// resolution snapshot with the approved snapshot, and lets you apply reviewed
/// changes.
///
/// The CLI is organized around a short workflow:
/// - create and maintain facts
/// - resolve current state
/// - inspect or diff snapshots
/// - apply approved changes
///
/// Use `ic <command> --help` for detailed help on any command.
#[derive(Parser)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) config: Config,

    #[command(subcommand)]
    pub(crate) command: Command,
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Init(InitCatalogArgs),
    Add(AddFactArgs),
    Edit(EditFactArgs),
    Rename(RenameFactArgs),
    #[command(alias = "rm")]
    Remove(RemoveFactArgs),
    #[command(alias = "sh")]
    Show(ShowFactArgs),
    #[command(alias = "ls")]
    List(ListFactArgs),
    #[command(subcommand, name = "op")]
    Operation(OperationCommand),
    #[command(alias = "r")]
    Resolve(ResolveArgs),
    #[command(alias = "i")]
    Inspect(InspectArgs),
    #[command(alias = "d")]
    Diff(DiffArgs),
    #[command(alias = "c")]
    Check(CheckArgs),
    #[command(alias = "up")]
    Apply(ApplyArgs),
}
