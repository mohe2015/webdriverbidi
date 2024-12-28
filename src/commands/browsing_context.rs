use crate::commands::id;
use crate::models::remote::browsing_context::{GetTree, GetTreeParameters, Navigate, NavigateParameters};
use crate::models::local::browsing_context::{GetTreeResult, NavigateResult};
use serde::{Deserialize, Serialize};
use crate::session::WebDriverBiDiSession;
    
#[derive(Debug, Serialize, Deserialize)]
struct NavigateCommand {
    id: u64,
    #[serde(flatten)]
    navigate: Navigate,
}

impl NavigateCommand {
    /// Builds the `browsingContext.navigate` command
    pub fn new(params: NavigateParameters) -> Self {
        Self {
            id: id::get_next_id(),
            navigate: Navigate::new(params),
        }
    }
}

pub async fn navigate(session: &mut WebDriverBiDiSession, params: NavigateParameters) -> Result<NavigateResult, Box<dyn std::error::Error>> {
    let navigate_cmd = NavigateCommand::new(params);
    
    session
        .send_command::<NavigateCommand, NavigateResult>(navigate_cmd).await
}

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct GetTreeCommand {
    id: u64,
    #[serde(flatten)]
    get_tree: GetTree,
}

impl GetTreeCommand {
    /// Builds the `browsingContext.getTree` command
    pub fn new(get_tree_params: GetTreeParameters) -> Self {
        Self {
            id: id::get_next_id(),
            get_tree: GetTree::new(get_tree_params),
        }
    }
}

pub async fn get_tree(session: &mut WebDriverBiDiSession, get_tree_params: GetTreeParameters) -> Result<GetTreeResult, Box<dyn std::error::Error>> {
    let get_tree_cmd = GetTreeCommand::new(get_tree_params);
    
    session
        .send_command::<GetTreeCommand, GetTreeResult>(get_tree_cmd).await
}