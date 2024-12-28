use thiserror::Error;
use crate::models::local::browsing_context::{GetTreeResult, NavigateResult};
use crate::models::remote::browsing_context::{GetTree, GetTreeParameters, Navigate, NavigateParameters};
use crate::session::WebDriverBiDiSession;
use serde::{Deserialize, Serialize};

use super::id;

#[derive(Debug, Error)]
pub enum BrowsingContextError {
    #[error("Failed to send command: {0}")]
    CommandError(String),
}

// --------------------------------------------------

/// Represents the `browsingContext.navigate` command.
#[derive(Debug, Serialize, Deserialize)]
struct NavigateCommand {
    id: u64,
    #[serde(flatten)]
    navigate: Navigate,
}

impl NavigateCommand {
    /// Constructs a new `NavigateCommand` with a unique ID and the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for the navigate command.
    ///
    /// # Returns
    ///
    /// A new instance of `NavigateCommand`.
    fn new(params: NavigateParameters) -> Self {
        Self {
            id: id::get_next_id(),
            navigate: Navigate::new(params),
        }
    }
}

/// Sends a `browsingContext.navigate` command to the WebDriver BiDi session.
///
/// # Arguments
///
/// * `session` - A mutable reference to the WebDriver BiDi session.
/// * `params` - Parameters for the navigate command.
///
/// # Returns
///
/// A `Result` containing either a `NavigateResult` or a `WebDriverError`.
pub async fn navigate(
    session: &mut WebDriverBiDiSession,
    params: NavigateParameters,
) -> Result<NavigateResult, BrowsingContextError> {
    let navigate_cmd = NavigateCommand::new(params);

    session
        .send_command::<NavigateCommand, NavigateResult>(navigate_cmd)
        .await
        .map_err(|e| BrowsingContextError::CommandError(e.to_string()))
}

// --------------------------------------------------

/// Represents the `browsingContext.getTree` command.
#[derive(Debug, Serialize, Deserialize)]
struct GetTreeCommand {
    id: u64,
    #[serde(flatten)]
    get_tree: GetTree,
}

impl GetTreeCommand {
    /// Constructs a new `GetTreeCommand` with a unique ID and the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `get_tree_params` - Parameters for the getTree command.
    ///
    /// # Returns
    ///
    /// A new instance of `GetTreeCommand`.
    fn new(get_tree_params: GetTreeParameters) -> Self {
        Self {
            id: id::get_next_id(),
            get_tree: GetTree::new(get_tree_params),
        }
    }
}

/// Sends a `browsingContext.getTree` command to the WebDriver BiDi session.
///
/// # Arguments
///
/// * `session` - A mutable reference to the WebDriver BiDi session.
/// * `get_tree_params` - Parameters for the getTree command.
///
/// # Returns
///
/// A `Result` containing either a `GetTreeResult` or a `WebDriverError`.
pub async fn get_tree(
    session: &mut WebDriverBiDiSession,
    get_tree_params: GetTreeParameters,
) -> Result<GetTreeResult, BrowsingContextError> {
    let get_tree_cmd = GetTreeCommand::new(get_tree_params);

    session
        .send_command::<GetTreeCommand, GetTreeResult>(get_tree_cmd)
        .await
        .map_err(|e| BrowsingContextError::CommandError(e.to_string()))
}