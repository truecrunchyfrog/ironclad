pub(crate) mod ack;
pub(crate) mod audit;
pub(crate) mod cell;
pub(crate) mod dependency;
pub(crate) mod cluster;
pub(crate) mod operation;
pub(crate) mod pipeline;

use clap::{Parser, Subcommand};

use crate::{
    args::{
        ack::AckArgs, audit::AuditArgs, cell::CellCommand, dependency::DependencyCommand,
        cluster::ClusterCommand, operation::OperationCommand, pipeline::PipelineCommand,
    },
    config::Config,
};

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
    #[command(subcommand)]
    Cluster(ClusterCommand),

    #[command(subcommand)]
    Cell(CellCommand),

    #[command(subcommand, name = "dep")]
    Dependency(DependencyCommand),

    #[command(subcommand, name = "plan")]
    Pipeline(PipelineCommand),

    #[command(subcommand, name = "op")]
    Operation(OperationCommand),

    Audit(AuditArgs),

    Ack(AckArgs),
}
