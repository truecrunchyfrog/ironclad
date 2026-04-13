pub(crate) mod audit;
pub(crate) mod fact;
pub(crate) mod cluster;
pub(crate) mod dependency;
pub(crate) mod operation;
pub(crate) mod review;
pub(crate) mod schema;

use clap::{Parser, Subcommand};

use crate::{
    args::{
        audit::AuditArgs, fact::FactCommand, cluster::ClusterCommand,
        dependency::DependencyCommand, operation::OperationCommand, review::ReviewArgs,
        schema::SchemaCommand,
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
    Fact(FactCommand),

    #[command(subcommand, name = "op")]
    Operation(OperationCommand),

    #[command(subcommand)]
    Schema(SchemaCommand),

    #[command(subcommand, name = "dep")]
    Dependency(DependencyCommand),

    Audit(AuditArgs),

    Review(ReviewArgs),
}
