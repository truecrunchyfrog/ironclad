use clap::{ArgGroup, Args};

/// Create dependencies.
#[derive(Args)]
#[command(group(
    ArgGroup::new("dependency_kind")
        .args(["dependency", "from", "mirror"])
        .required(true)
))]
pub(crate) struct AddDependencyArgs {
    /// ID of cell to add dependencies to.
    #[arg(default_value = "-")]
    pub(crate) cell_id: Vec<String>,

    /// ID of cell to add as dependency.
    #[arg(short, long, required = true)]
    pub(crate) dependency: Vec<String>,

    /// ID of cell to copy all dependencies from to add.
    #[arg(long)]
    pub(crate) from: Vec<String>,

    /// Make the dependents dependencies of each other (when there are more than one dependents).
    #[arg(long)]
    pub(crate) mirror: bool,
}
