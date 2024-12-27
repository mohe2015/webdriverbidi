use serde::{Deserialize, Serialize};

use crate::models::remote::browsing_context::{GetTree, Navigate};


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

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTreeCommand {
    id: u64,
    #[serde(flatten)]
    get_tree: GetTree,
}

impl GetTreeCommand {
    /// Builds the `browsingContext.getTree` command
    pub fn new(id: u64, get_tree: GetTree) -> Self {
        Self { id, get_tree }
    }
}
