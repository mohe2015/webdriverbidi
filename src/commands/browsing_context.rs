use log::{debug, error};
use serde::{Deserialize, Serialize};

// --------------------------------------------------

use super::id;
use crate::error::CommandError;
use crate::models::local::browsing_context::{
    GetTreeResult, NavigateResult, TraverseHistoryResult, CaptureScreenshotResult
};
use crate::models::local::result_data::EmptyResult;
use crate::models::remote::browsing_context::{
    ActivateParameters, GetTree, GetTreeParameters, Navigate, NavigateParameters, TraverseHistory,
    TraverseHistoryParameters, CaptureScreenshotParameters, CloseParameters
};
use crate::session::WebDriverBiDiSession;

// --------------------------------------------------

/// Sends a command to the WebDriver BiDi session and processes the result.
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

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-activate

/// Represents the `browsingContext.activate` command.
#[derive(Debug, Serialize, Deserialize)]
struct ActivateCommand {
    id: u64,
    #[serde(flatten)]
    activate: ActivateParameters,
}

impl ActivateCommand {
    /// Constructs a new `ActivateCommand` with a unique ID and the provided parameters.
    fn new(params: ActivateParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating ActivateCommand with id: {}", id);
        Self {
            id,
            activate: params,
        }
    }
}

/// Sends a `browsingContext.activate` command to the WebDriver BiDi session.
pub async fn activate(
    session: &mut WebDriverBiDiSession,
    params: ActivateParameters,
) -> Result<EmptyResult, CommandError> {
    let activate_cmd = ActivateCommand::new(params);
    send_command(session, activate_cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-captureScreenshot

/// Represents the `browsingContext.captureScreenshot` command.
#[derive(Debug, Serialize, Deserialize)]
struct CaptureScreenshotCommand {
    id: u64,
    #[serde(flatten)]
    capture_screenshot: CaptureScreenshotParameters,
}

impl CaptureScreenshotCommand {
    /// Constructs a new `CaptureScreenshotCommand` with a unique ID and the provided parameters.
    fn new(params: CaptureScreenshotParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating CaptureScreenshotCommand with id: {}", id);
        Self {
            id,
            capture_screenshot: params,
        }
    }
}

/// Sends a `browsingContext.captureScreenshot` command to the WebDriver BiDi session.
pub async fn capture_screenshot(
    session: &mut WebDriverBiDiSession,
    params: CaptureScreenshotParameters,
) -> Result<CaptureScreenshotResult, CommandError> {
    let capture_screenshot_cmd = CaptureScreenshotCommand::new(params);
    send_command(session, capture_screenshot_cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-close

/// Represents the `browsingContext.close` command.
#[derive(Debug, Serialize, Deserialize)]
struct CloseCommand {
    id: u64,
    #[serde(flatten)]
    close: CloseParameters,
}

impl CloseCommand {
    /// Constructs a new `CloseCommand` with a unique ID and the provided parameters.
    fn new(params: CloseParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating CloseCommand with id: {}", id);
        Self {
            id,
            close: params,
        }
    }
}

/// Sends a `browsingContext.close` command to the WebDriver BiDi session.
pub async fn close(
    session: &mut WebDriverBiDiSession,
    params: CloseParameters,
) -> Result<EmptyResult, CommandError> {
    let close_cmd = CloseCommand::new(params);
    send_command(session, close_cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-create



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
pub async fn traverse_history(
    session: &mut WebDriverBiDiSession,
    params: TraverseHistoryParameters,
) -> Result<TraverseHistoryResult, CommandError> {
    let traverse_history_cmd = TraverseHistoryCommand::new(params);
    send_command(session, traverse_history_cmd).await
}

// --------------------------------------------------
