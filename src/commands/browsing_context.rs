use crate::commands::id;
use crate::models::remote::browsing_context::{GetTree, Navigate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigateCommand {
    id: u64,
    #[serde(flatten)]
    navigate: Navigate,
}

impl NavigateCommand {
    /// Builds the `browsingContext.navigate` command
    pub fn new(navigate: Navigate) -> Self {
        Self {
            id: id::get_next_id(),
            navigate,
        }
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
    pub fn new(get_tree: GetTree) -> Self {
        Self {
            id: id::get_next_id(),
            get_tree,
        }
    }
}
