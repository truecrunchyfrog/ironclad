use clap::Args;

/// Show an operation.
#[derive(Args)]
pub(crate) struct ShowOperationArgs {
    /// ID of operation to show.
    pub(crate) operation_id: String,
}
