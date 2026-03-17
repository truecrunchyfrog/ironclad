pub(crate) mod ack;
pub(crate) mod audit;
pub(crate) mod dependency;
pub(crate) mod ledger;
pub(crate) mod node;
pub(crate) mod operation;
pub(crate) mod pipeline;

use clap::{Args, Parser, Subcommand};

use crate::args::{
    ack::AckArgs, audit::AuditArgs, dependency::DependencyCommand, ledger::LedgerCommand,
    node::NodeCommand, operation::OperationCommand, pipeline::PipelineCommand,
};

#[derive(Parser)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) global: GlobalArgs,

    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Args)]
pub(crate) struct GlobalArgs {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    #[command(subcommand, alias = "l")]
    Ledger(LedgerCommand),

    #[command(subcommand, alias = "n")]
    Node(NodeCommand),

    #[command(subcommand, name = "dep", alias = "d")]
    Dependency(DependencyCommand),

    #[command(subcommand, alias = "p")]
    Pipeline(PipelineCommand),

    #[command(subcommand, alias = "o")]
    Operation(OperationCommand),

    #[command(alias = "a")]
    Audit(AuditArgs),

    #[command(alias = "k")]
    Ack(AckArgs),
}

pub(crate) fn parse() -> Cli {
    Cli::parse()
}
