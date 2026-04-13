use clap::Args;

/// List a schema's stages.
#[derive(Args)]
pub(crate) struct ListSchemaArgs {
    /// ID of fact to show schema of.
    pub(crate) fact_id: Option<String>,

    /// Show the schema's raw JSON array.
    #[arg(short, long)]
    pub(crate) raw: bool,
}
