use log::debug;
use serde::{Deserialize, Serialize};

// --------------------------------------------------

use super::id;
use super::utils;
use crate::error::CommandError;
use crate::local::browser::ClientWindowInfo;
use crate::local::browser::*;
use crate::local::result_data::EmptyResult;
use crate::remote::browser::*;
use crate::remote::EmptyParams;
use crate::session::WebDriverBiDiSession;

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-close

/// Represents the `browser.close` command.
#[derive(Debug, Serialize, Deserialize)]
struct CloseCommand {
    id: u64,
    #[serde(flatten)]
    close: Close,
}

impl CloseCommand {
    /// Constructs a new `CloseCommand` with a unique ID and the provided parameters.
    fn new(params: EmptyParams) -> Self {
        let id = id::get_next_id();
        debug!("Creating CloseCommand with id: {}", id);
        Self {
            id,
            close: Close::new(params),
        }
    }
}

/// Sends a `browser.close` command to the WebDriver BiDi session.
pub async fn close(
    session: &mut WebDriverBiDiSession,
    params: EmptyParams,
) -> Result<EmptyResult, CommandError> {
    let status_cmd = CloseCommand::new(params);
    utils::send_command(session, status_cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-createUserContext

/// Represents the `browser.createUserContext` command.
#[derive(Debug, Serialize, Deserialize)]
struct CreateUserContextCommand {
    id: u64,
    #[serde(flatten)]
    create_user_context: CreateUserContext,
}

impl CreateUserContextCommand {
    /// Constructs a new `CreateUserContextCommand` with a unique ID and the provided parameters.
    fn new(params: EmptyParams) -> Self {
        let id = id::get_next_id();
        debug!("Creating CreateUserContextCommand with id: {}", id);
        Self {
            id,
            create_user_context: CreateUserContext::new(params),
        }
    }
}

/// Sends a `browser.createUserContext` command to the WebDriver BiDi session.
pub async fn create_user_context(
    session: &mut WebDriverBiDiSession,
    params: EmptyParams,
) -> Result<CreateUserContextResult, CommandError> {
    let cmd = CreateUserContextCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-getClientWindows

/// Represents the `browser.getClientWindows` command.
#[derive(Debug, Serialize, Deserialize)]
struct GetClientWindowsCommand {
    id: u64,
    #[serde(flatten)]
    get_client_windows: GetClientWindows,
}

impl GetClientWindowsCommand {
    /// Constructs a new `GetClientWindowsCommand` with a unique ID and the provided parameters.
    fn new(params: EmptyParams) -> Self {
        let id = id::get_next_id();
        debug!("Creating GetClientWindowsCommand with id: {}", id);
        Self {
            id,
            get_client_windows: GetClientWindows::new(params),
        }
    }
}

/// Sends a `browser.getClientWindows` command to the WebDriver BiDi session.
pub async fn get_client_windows(
    session: &mut WebDriverBiDiSession,
    params: EmptyParams,
) -> Result<GetClientWindowsResult, CommandError> {
    let cmd = GetClientWindowsCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-getUserContexts

/// Represents the `browser.getUserContexts` command.
#[derive(Debug, Serialize, Deserialize)]
struct GetUserContextsCommand {
    id: u64,
    #[serde(flatten)]
    get_user_contexts: GetUserContexts,
}

impl GetUserContextsCommand {
    /// Constructs a new `GetUserContextsCommand` with a unique ID and the provided parameters.
    fn new(params: EmptyParams) -> Self {
        let id = id::get_next_id();
        debug!("Creating GetUserContextsCommand with id: {}", id);
        Self {
            id,
            get_user_contexts: GetUserContexts::new(params),
        }
    }
}

/// Sends a `browser.getUserContexts` command to the WebDriver BiDi session.
pub async fn get_user_contexts(
    session: &mut WebDriverBiDiSession,
    params: EmptyParams,
) -> Result<GetUserContextsResult, CommandError> {
    let cmd = GetUserContextsCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-removeUserContext

/// Represents the `browser.removeUserContext` command.
#[derive(Debug, Serialize, Deserialize)]
struct RemoveUserContextCommand {
    id: u64,
    #[serde(flatten)]
    remove_user_context: RemoveUserContext,
}

impl RemoveUserContextCommand {
    /// Constructs a new `RemoveUserContextCommand` with a unique ID and the provided parameters.
    fn new(params: RemoveUserContextParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating RemoveUserContextCommand with id: {}", id);
        Self {
            id,
            remove_user_context: RemoveUserContext::new(params),
        }
    }
}

/// Sends a `browser.removeUserContext` command to the WebDriver BiDi session.
pub async fn remove_user_context(
    session: &mut WebDriverBiDiSession,
    params: RemoveUserContextParameters,
) -> Result<EmptyResult, CommandError> {
    let cmd = RemoveUserContextCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browser-setClientWindowState

/// Represents the `browser.setClientWindowState` command.
#[derive(Debug, Serialize, Deserialize)]
struct SetClientWindowStateCommand {
    id: u64,
    #[serde(flatten)]
    set_client_window_state: SetClientWindowState,
}

impl SetClientWindowStateCommand {
    /// Constructs a new `SetClientWindowStateCommand` with a unique ID and the provided parameters.
    fn new(params: SetClientWindowStateParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating SetClientWindowStateCommand with id: {}", id);
        Self {
            id,
            set_client_window_state: SetClientWindowState::new(params),
        }
    }
}

/// Sends a `browser.setClientWindowState` command to the WebDriver BiDi session.
pub async fn set_client_window_state(
    session: &mut WebDriverBiDiSession,
    params: SetClientWindowStateParameters,
) -> Result<ClientWindowInfo, CommandError> {
    let cmd = SetClientWindowStateCommand::new(params);
    utils::send_command(session, cmd).await
}
