#[derive(thiserror::Error, Debug)]
pub(crate) enum FragmentError {
    #[error("no element")]
    NoElement,
}
