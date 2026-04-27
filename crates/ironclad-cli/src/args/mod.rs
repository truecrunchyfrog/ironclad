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

#[derive(Parser)]
#[command(
    name = "ic",
    about = "Track and review small pieces of external state.",
    long_about = "Track and review small pieces of external state.\n\n\
Ironclad stores facts in a catalog, resolves them into snapshots, compares the \
resolution snapshot with the approved snapshot, and lets you apply reviewed \
changes.\n\n\
The CLI is organized around a short workflow:\n\
- create and maintain facts\n\
- resolve current state\n\
- inspect or diff snapshots\n\
- apply approved changes\n\n\
Use `ic <command> --help` for detailed help on any command."
)]
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
    #[command(
        about = "Create a catalog directory.",
        long_about = "Create a catalog directory.\n\n\
`init` prepares the `.ironclad/` directory and its initial files. By default \
it creates the catalog directory in the current working directory.\n\n\
Use `--dir` to choose another location. If the path ends with `.ironclad`, \
that exact directory is created. Otherwise Ironclad creates `.ironclad/` \
inside the given directory."
    )]
    Init(InitCatalogArgs),
    #[command(
        about = "Create a fact file.",
        long_about = "Create a fact file.\n\n\
Facts are the units Ironclad resolves into snapshots. A normal `add` command \
creates a fact file and indexes it under a label. With `--no-index`, the fact \
is created without a label and is addressed later by fact ID.\n\n\
Indexed facts are the normal case. Unindexed facts are mainly useful for \
experiments or one-off work."
    )]
    Add(AddFactArgs),
    #[command(
        about = "Open a fact in your editor.",
        long_about = "Open a fact in your editor.\n\n\
`edit` resolves the given fact selector, finds the underlying TOML file, and \
opens it with `$EDITOR`. The selector can be a label or a fact ID.\n\n\
Use this command when you want to change a fact pipeline, add imports or \
exports, or update the description."
    )]
    Edit(EditFactArgs),
    #[command(
        about = "Rename an indexed fact label.",
        long_about = "Rename an indexed fact label.\n\n\
`rename` changes the label stored in `index.toml`. The fact file itself keeps \
the same fact ID.\n\n\
You can select the fact by label or by fact ID. The new label must not already \
belong to a different fact."
    )]
    Rename(RenameFactArgs),
    #[command(alias = "rm")]
    #[command(
        about = "Remove a fact.",
        long_about = "Remove a fact.\n\n\
`remove` deletes the fact file. If the fact is indexed, the matching label is \
also removed from `index.toml`.\n\n\
The selector can be a label or a fact ID."
    )]
    Remove(RemoveFactArgs),
    #[command(alias = "sh")]
    #[command(
        about = "Show a fact or its path.",
        long_about = "Show a fact or its path.\n\n\
By default `show` prints a structured view of the fact definition. With \
`--path`, it prints the path to the fact file instead.\n\n\
The selector can be a label or a fact ID."
    )]
    Show(ShowFactArgs),
    #[command(alias = "ls")]
    #[command(
        about = "List indexed facts.",
        long_about = "List indexed facts.\n\n\
`list` prints the labels currently stored in `index.toml`. With `--verbose`, \
it also prints the fact description next to each label.\n\n\
This command only shows indexed facts. Unindexed facts remain addressable by \
fact ID but do not appear in the list."
    )]
    List(ListFactArgs),
    #[command(subcommand, name = "op")]
    #[command(
        about = "Inspect and evaluate operations.",
        long_about = "Inspect and evaluate operations.\n\n\
The `op` command family is for operation-level work: listing available \
operations, showing one operation in detail, and evaluating an operation \
without writing a fact.\n\n\
This is useful when you are designing a pipeline or debugging one step at a \
time."
    )]
    Operation(OperationCommand),
    #[command(alias = "r")]
    #[command(
        about = "Resolve facts into a snapshot.",
        long_about = "Resolve facts into a snapshot.\n\n\
`resolve` evaluates fact pipelines and writes the resulting resolution snapshot. \
Without arguments it resolves all indexed facts.\n\n\
You can limit the run to selected labels, exclude selected labels, choose a \
different output location, and disable secret redaction."
    )]
    Resolve(ResolveArgs),
    #[command(alias = "i")]
    #[command(
        about = "Inspect a snapshot.",
        long_about = "Inspect a snapshot.\n\n\
Without a label, `inspect` prints one summary line per fact in the snapshot. \
With a label, it shows the samples for that fact in a structured form.\n\n\
By default the command reads the approved snapshot from `canon.json`, but you \
can point it at another file or stdin."
    )]
    Inspect(InspectArgs),
    #[command(alias = "d")]
    #[command(
        about = "Compare two snapshots.",
        long_about = "Compare two snapshots.\n\n\
Without a label, `diff` prints a compact fact-level summary of changes. With a \
label, it shows structured sample-level changes for one fact.\n\n\
By default the command compares the resolution snapshot in `actual.json` with the \
approved snapshot in `canon.json`, but both can be overridden."
    )]
    Diff(DiffArgs),
    #[command(alias = "c")]
    #[command(
        about = "Check whether two snapshots are identical.",
        long_about = "Check whether two snapshots are identical.\n\n\
`check` is the non-interactive summary form of `diff`. It compares two \
snapshots, prints whether drift exists, and exits with status `0` when nothing \
drifted or `1` when any fact drifted.\n\n\
This is the command intended for CI or shell scripts."
    )]
    Check(CheckArgs),
    #[command(alias = "up")]
    #[command(
        about = "Apply changes into the approved snapshot.",
        long_about = "Apply changes into the approved snapshot.\n\n\
`apply` promotes facts from the resolution snapshot into the approved snapshot. \
You can apply selected labels or replace the entire approved snapshot with \
`--all`.\n\n\
This is the final step in the review loop after resolving, inspecting, and \
approving drift."
    )]
    Apply(ApplyArgs),
}
