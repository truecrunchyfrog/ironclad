use clap::Args;

/// Show one operation in detail.
///
/// `op show` prints the operation's description together with a TOML template for
/// its options, when it has options.
///
/// Use this command to understand an operation before using it in a fact or with
/// `op eval`.
#[derive(Args)]
pub(crate) struct ShowOperationArgs {
    /// Operation ID to show.
    pub(crate) operation_id: String,
}
