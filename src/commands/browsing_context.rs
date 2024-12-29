use super::id;
use crate::error::CommandError;
use crate::models::local::browsing_context::{GetTreeResult, NavigateResult};
use crate::models::remote::browsing_context::{
    GetTree, GetTreeParameters, Navigate, NavigateParameters,
};
use crate::session::WebDriverBiDiSession;
use serde::{Deserialize, Serialize};

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
/// A `Result` containing either a `NavigateResult` or a `CommandError`.
pub async fn navigate(
    session: &mut WebDriverBiDiSession,
    params: NavigateParameters,
) -> Result<NavigateResult, CommandError> {
    let navigate_cmd = NavigateCommand::new(params);

    let result = session
        .send_command::<NavigateCommand, NavigateResult>(navigate_cmd)
        .await?;

    Ok(result)
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
    /// * `params` - Parameters for the getTree command.
    ///
    /// # Returns
    ///
    /// A new instance of `GetTreeCommand`.
    fn new(params: GetTreeParameters) -> Self {
        Self {
            id: id::get_next_id(),
            get_tree: GetTree::new(params),
        }
    }
}

/// Sends a `browsingContext.getTree` command to the WebDriver BiDi session.
///
/// # Arguments
///
/// * `session` - A mutable reference to the WebDriver BiDi session.
/// * `params` - Parameters for the getTree command.
///
/// # Returns
///
/// A `Result` containing either a `GetTreeResult` or a `CommandError`.
pub async fn get_tree(
    session: &mut WebDriverBiDiSession,
    params: GetTreeParameters,
) -> Result<GetTreeResult, CommandError> {
    let get_tree_cmd = GetTreeCommand::new(params);

    let rslt = session
        .send_command::<GetTreeCommand, GetTreeResult>(get_tree_cmd)
        .await?;

    Ok(rslt)
}
