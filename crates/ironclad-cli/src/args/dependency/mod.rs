pub(crate) mod add;
pub(crate) mod remove;

use clap::Subcommand;

use crate::args::dependency::{add::AddDependencyArgs, remove::RemoveDependencyArgs};

/// Manage logical dependencies between cells.
#[derive(Subcommand)]
pub(crate) enum DependencyCommand {
    Add(AddDependencyArgs),
    Remove(RemoveDependencyArgs),
}
