use serde::{Deserialize, Serialize};

use crate::models::remote::browsing_context::Navigate;

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigateCommand {
    id: u64,
    #[serde(flatten)]
    navigate: Navigate,
}

impl NavigateCommand {
    /// Builds the `browsingContext.navigate` command
    pub fn new(id: u64, navigate: Navigate) -> Self {
        Self { id, navigate }
    }
}
