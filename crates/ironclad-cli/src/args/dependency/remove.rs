use clap::{ArgGroup, Args};

/// Remove dependencies.
#[derive(Args)]
#[command(group(
    ArgGroup::new("depencency_kind")
        .args(["dependency", "all"])
        .required(true)
        .multiple(false)
))]
pub(crate) struct RemoveDependencyArgs {
    /// ID of dependent fact to remove dependency from.
    #[arg(default_value = "-")]
    pub(crate) fact_id: Vec<String>,

    /// ID of dependency fact to remove.
    #[arg(short, long)]
    pub(crate) dependency: Vec<String>,

    /// Remove all dependencies.
    #[arg(short, long)]
    pub(crate) all: bool,
}
