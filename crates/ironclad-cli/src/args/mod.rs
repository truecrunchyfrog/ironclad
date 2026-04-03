pub(crate) mod ack;
pub(crate) mod audit;
pub(crate) mod cell;
pub(crate) mod cluster;
pub(crate) mod dependency;
pub(crate) mod operation;
pub(crate) mod schema;

use clap::{Parser, Subcommand};

use crate::{
    args::{
        ack::AckArgs, audit::AuditArgs, cell::CellCommand, cluster::ClusterCommand,
        dependency::DependencyCommand, operation::OperationCommand, schema::SchemaCommand,
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

    #[command(subcommand, name = "schema")]
    Schema(SchemaCommand),

    #[command(subcommand, name = "op")]
    Operation(OperationCommand),

    Audit(AuditArgs),

    Ack(AckArgs),
}
