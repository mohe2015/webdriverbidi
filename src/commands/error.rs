use thiserror::Error;
use super::browsing_context;

#[derive(Debug, Error)]
pub enum BiDiError {
    #[error("Input error: {0}")]
    InputError(#[from] browsing_context::BrowsingContextError),
    #[error("Unknown error: {0}")]
    UnknownError(String),
}