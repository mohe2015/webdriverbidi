use log::{debug, error};
use serde::{Deserialize, Serialize};

// --------------------------------------------------

use super::id;
use crate::error::CommandError;
use crate::models::local::browsing_context::*;
use crate::models::local::result_data::EmptyResult;
use crate::models::remote::browsing_context::*;
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
        Self { id, close: params }
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

/// Represents the `browsingContext.create` command.
#[derive(Debug, Serialize, Deserialize)]
struct CreateCommand {
    id: u64,
    #[serde(flatten)]
    create: CreateParameters,
}

impl CreateCommand {
    /// Constructs a new `CreateCommand` with a unique ID and the provided parameters.
    fn new(params: CreateParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating CreateCommand with id: {}", id);
        Self { id, create: params }
    }
}

/// Sends a `browsingContext.create` command to the WebDriver BiDi session.
pub async fn create(
    session: &mut WebDriverBiDiSession,
    params: CreateParameters,
) -> Result<CreateResult, CommandError> {
    let create_cmd = CreateCommand::new(params);
    send_command(session, create_cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-getTree

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

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-handleUserPrompt

/// Represents the `browsingContext.handleUserPrompt` command.
#[derive(Debug, Serialize, Deserialize)]
struct HandleUserPromptCommand {
    id: u64,
    #[serde(flatten)]
    handle_user_prompt: HandleUserPrompt,
}

impl HandleUserPromptCommand {
    /// Constructs a new `HandleUserPromptCommand` with a unique ID and the provided parameters.
    fn new(params: HandleUserPromptParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating HandleUserPromptCommand with id: {}", id);
        Self {
            id,
            handle_user_prompt: HandleUserPrompt::new(params),
        }
    }
}

/// Sends a `browsingContext.handleUserPrompt` command to the WebDriver BiDi session.
pub async fn handle_user_prompt(
    session: &mut WebDriverBiDiSession,
    params: HandleUserPromptParameters,
) -> Result<EmptyResult, CommandError> {
    let handle_user_prompt_cmd = HandleUserPromptCommand::new(params);
    send_command(session, handle_user_prompt_cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-locateNodes

/// Represents the `browsingContext.locateNodes` command.
#[derive(Debug, Serialize, Deserialize)]
struct LocateNodesCommand {
    id: u64,
    #[serde(flatten)]
    locate_nodes: LocateNodes,
}

impl LocateNodesCommand {
    /// Constructs a new `LocateNodesCommand` with a unique ID and the provided parameters.
    fn new(params: LocateNodesParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating LocateNodesCommand with id: {}", id);
        Self {
            id,
            locate_nodes: LocateNodes::new(params),
        }
    }
}

/// Sends a `browsingContext.locateNodes` command to the WebDriver BiDi session.
pub async fn locate_nodes(
    session: &mut WebDriverBiDiSession,
    params: LocateNodesParameters,
) -> Result<LocateNodesResult, CommandError> {
    let locate_nodes_cmd = LocateNodesCommand::new(params);
    send_command(session, locate_nodes_cmd).await
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

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-print

/// Represents the `browsingContext.print` command.
#[derive(Debug, Serialize, Deserialize)]
struct PrintCommand {
    id: u64,
    #[serde(flatten)]
    print: Print,
}

impl PrintCommand {
    /// Constructs a new `PrintCommand` with a unique ID and the provided parameters.
    fn new(params: PrintParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating PrintCommand with id: {}", id);
        Self {
            id,
            print: Print::new(params),
        }
    }
}

/// Sends a `browsingContext.print` command to the WebDriver BiDi session.
pub async fn print(
    session: &mut WebDriverBiDiSession,
    params: PrintParameters,
) -> Result<PrintResult, CommandError> {
    let print_cmd = PrintCommand::new(params);
    send_command(session, print_cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-reload

/// Represents the `browsingContext.reload` command.
#[derive(Debug, Serialize, Deserialize)]
struct ReloadCommand {
    id: u64,
    #[serde(flatten)]
    reload: Reload,
}

impl ReloadCommand {
    /// Constructs a new `ReloadCommand` with a unique ID and the provided parameters.
    fn new(params: ReloadParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating ReloadCommand with id: {}", id);
        Self {
            id,
            reload: Reload::new(params),
        }
    }
}

/// Sends a `browsingContext.reload` command to the WebDriver BiDi session.
pub async fn reload(
    session: &mut WebDriverBiDiSession,
    params: ReloadParameters,
) -> Result<NavigateResult, CommandError> {
    let reload_cmd = ReloadCommand::new(params);
    send_command(session, reload_cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-setViewport

/// Represents the `browsingContext.setViewport` command.
#[derive(Debug, Serialize, Deserialize)]
struct SetViewportCommand {
    id: u64,
    #[serde(flatten)]
    set_viewport: SetViewport,
}

impl SetViewportCommand {
    /// Constructs a new `SetViewportCommand` with a unique ID and the provided parameters.
    fn new(params: SetViewportParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating SetViewportCommand with id: {}", id);
        Self {
            id,
            set_viewport: SetViewport::new(params),
        }
    }
}

/// Sends a `browsingContext.setViewport` command to the WebDriver BiDi session.
pub async fn set_viewport(
    session: &mut WebDriverBiDiSession,
    params: SetViewportParameters,
) -> Result<EmptyResult, CommandError> {
    let set_viewport_cmd = SetViewportCommand::new(params);
    send_command(session, set_viewport_cmd).await
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
