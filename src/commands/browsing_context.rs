use log::{debug, error};
use serde::{Deserialize, Serialize};

// --------------------------------------------------

use super::id;
use crate::error::CommandError;
use crate::models::local::browsing_context::{GetTreeResult, NavigateResult, TraverseHistoryResult};
use crate::models::remote::browsing_context::{
    GetTree, GetTreeParameters, Navigate, NavigateParameters, TraverseHistory, TraverseHistoryParameters
};
use crate::session::WebDriverBiDiSession;

// --------------------------------------------------

/// Sends a command to the WebDriver BiDi session and processes the result.
///
/// # Arguments
///
/// * `session` - A mutable reference to the WebDriver BiDi session.
/// * `command` - The command to send.
///
/// # Returns
///
/// A `Result` containing either the command result or a `CommandError`.
async fn send_command<C, R>(
    session: &mut WebDriverBiDiSession,
    command: C,
) -> Result<R, CommandError>
where
    C: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let command_id = id::get_next_id();
    debug!("Sending command with id: {}", command_id); // Log before sending command

    match session.send_command::<C, R>(command).await {
        Ok(rslt) => {
            debug!("Command with id: {} succeeded", command_id); // Log success
            Ok(rslt)
        }
        Err(e) => {
            error!("Command with id: {} failed: {:?}", command_id, e); // Log error
            Err(e)
        }
    }
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
        let id = id::get_next_id();
        debug!("Creating NavigateCommand with id: {}", id);
        Self {
            id,
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
    send_command(session, navigate_cmd).await
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
        let id = id::get_next_id();
        debug!("Creating GetTreeCommand with id: {}", id);
        Self {
            id,
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
    send_command(session, get_tree_cmd).await
}

// --------------------------------------------------

/// Represents the `browsingContext.traverseHistory` command.
#[derive(Debug, Serialize, Deserialize)]
struct TraverseHistoryCommand {
    id: u64,
    #[serde(flatten)]
    traverse_history: TraverseHistory,
}

impl TraverseHistoryCommand {
    /// Constructs a new `TraverseHistoryCommand` with a unique ID and the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - Parameters for the traverseHistory command.
    ///
    /// # Returns
    ///
    /// A new instance of `TraverseHistoryCommand`.
    fn new(params: TraverseHistoryParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating TraverseHistoryCommand with id: {}", id);
        Self {
            id,
            traverse_history: TraverseHistory::new(params),
        }
    }
}

/// Sends a `browsingContext.traverseHistory` command to the WebDriver BiDi session.
///
/// # Arguments
///
/// * `session` - A mutable reference to the WebDriver BiDi session.
/// * `params` - Parameters for the traverseHistory command.
///
/// # Returns
///
/// A `Result` containing either a `TraverseHistoryResult` or a `CommandError`.
pub async fn traverse_history(
    session: &mut WebDriverBiDiSession,
    params: TraverseHistoryParameters,
) -> Result<TraverseHistoryResult, CommandError> {
    let traverse_history_cmd = TraverseHistoryCommand::new(params);
    send_command(session, traverse_history_cmd).await
}

// --------------------------------------------------
