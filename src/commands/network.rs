use log::debug;
use serde::{Deserialize, Serialize};

// --------------------------------------------------

use super::id;
use super::utils;
use crate::error::CommandError;
use crate::local::network::*;
use crate::local::result_data::EmptyResult;
use crate::remote::network::*;
use crate::session::WebDriverBiDiSession;

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-network-addIntercept

/// Represents the `network.addIntercept` command.
#[derive(Debug, Serialize, Deserialize)]
struct AddInterceptCommand {
    id: u64,
    #[serde(flatten)]
    add_intercept: AddIntercept,
}

impl AddInterceptCommand {
    /// Constructs a new `AddInterceptCommand` with a unique ID and the provided parameters.
    fn new(params: AddInterceptParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating AddInterceptCommand with id: {}", id);
        Self {
            id,
            add_intercept: AddIntercept::new(params),
        }
    }
}

/// Sends a `network.addIntercept` command to the WebDriver BiDi session.
pub async fn add_intercept(
    session: &mut WebDriverBiDiSession,
    params: AddInterceptParameters,
) -> Result<AddInterceptResult, CommandError> {
    let cmd = AddInterceptCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-network-continueRequest

/// Represents the `network.continueRequest` command.
#[derive(Debug, Serialize, Deserialize)]
struct ContinueRequestCommand {
    id: u64,
    #[serde(flatten)]
    continue_request: ContinueRequest,
}

impl ContinueRequestCommand {
    /// Constructs a new `ContinueRequestCommand` with a unique ID and the provided parameters.
    fn new(params: ContinueRequestParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating ContinueRequestCommand with id: {}", id);
        Self {
            id,
            continue_request: ContinueRequest::new(params),
        }
    }
}

/// Sends a `network.continueRequest` command to the WebDriver BiDi session.
pub async fn continue_request(
    session: &mut WebDriverBiDiSession,
    params: ContinueRequestParameters,
) -> Result<EmptyResult, CommandError> {
    let cmd = ContinueRequestCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-network-continueResponse

/// Represents the `network.continueResponse` command.
#[derive(Debug, Serialize, Deserialize)]
struct ContinueResponseCommand {
    id: u64,
    #[serde(flatten)]
    continue_response: ContinueResponse,
}

impl ContinueResponseCommand {
    /// Constructs a new `ContinueResponseCommand` with a unique ID and the provided parameters.
    fn new(params: ContinueResponseParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating ContinueResponseCommand with id: {}", id);
        Self {
            id,
            continue_response: ContinueResponse::new(params),
        }
    }
}

/// Sends a `network.continueResponse` command to the WebDriver BiDi session.
pub async fn continue_response(
    session: &mut WebDriverBiDiSession,
    params: ContinueResponseParameters,
) -> Result<EmptyResult, CommandError> {
    let cmd = ContinueResponseCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-network-continueWithAuth

/// Represents the `network.continueWithAuth` command.
#[derive(Debug, Serialize, Deserialize)]
struct ContinueWithAuthCommand {
    id: u64,
    #[serde(flatten)]
    continue_with_auth: ContinueWithAuth,
}

impl ContinueWithAuthCommand {
    /// Constructs a new `ContinueWithAuthCommand` with a unique ID and the provided parameters.
    fn new(params: ContinueWithAuthParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating ContinueWithAuthCommand with id: {}", id);
        Self {
            id,
            continue_with_auth: ContinueWithAuth::new(params),
        }
    }
}

/// Sends a `network.continueWithAuth` command to the WebDriver BiDi session.
pub async fn continue_with_auth(
    session: &mut WebDriverBiDiSession,
    params: ContinueWithAuthParameters,
) -> Result<EmptyResult, CommandError> {
    let cmd = ContinueWithAuthCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-network-failRequest

/// Represents the `network.failRequest` command.
#[derive(Debug, Serialize, Deserialize)]
struct FailRequestCommand {
    id: u64,
    #[serde(flatten)]
    fail_request: FailRequest,
}

impl FailRequestCommand {
    /// Constructs a new `FailRequestCommand` with a unique ID and the provided parameters.
    fn new(params: FailRequestParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating FailRequestCommand with id: {}", id);
        Self {
            id,
            fail_request: FailRequest::new(params),
        }
    }
}

/// Sends a `network.failRequest` command to the WebDriver BiDi session.
pub async fn fail_request(
    session: &mut WebDriverBiDiSession,
    params: FailRequestParameters,
) -> Result<EmptyResult, CommandError> {
    let cmd = FailRequestCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-network-provideResponse

/// Represents the `network.provideResponse` command.
#[derive(Debug, Serialize, Deserialize)]
struct ProvideResponseCommand {
    id: u64,
    #[serde(flatten)]
    provide_response: ProvideResponse,
}

impl ProvideResponseCommand {
    /// Constructs a new `ProvideResponseCommand` with a unique ID and the provided parameters.
    fn new(params: ProvideResponseParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating ProvideResponseCommand with id: {}", id);
        Self {
            id,
            provide_response: ProvideResponse::new(params),
        }
    }
}

/// Sends a `network.provideResponse` command to the WebDriver BiDi session.
pub async fn provide_response(
    session: &mut WebDriverBiDiSession,
    params: ProvideResponseParameters,
) -> Result<EmptyResult, CommandError> {
    let cmd = ProvideResponseCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-network-removeIntercept

/// Represents the `network.removeIntercept` command.
#[derive(Debug, Serialize, Deserialize)]
struct RemoveInterceptCommand {
    id: u64,
    #[serde(flatten)]
    remove_intercept: RemoveIntercept,
}

impl RemoveInterceptCommand {
    /// Constructs a new `RemoveInterceptCommand` with a unique ID and the provided parameters.
    fn new(params: RemoveInterceptParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating RemoveInterceptCommand with id: {}", id);
        Self {
            id,
            remove_intercept: RemoveIntercept::new(params),
        }
    }
}

/// Sends a `network.removeIntercept` command to the WebDriver BiDi session.
pub async fn remove_intercept(
    session: &mut WebDriverBiDiSession,
    params: RemoveInterceptParameters,
) -> Result<EmptyResult, CommandError> {
    let cmd = RemoveInterceptCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-network-setCacheBehavior

/// Represents the `network.setCacheBehavior` command.
#[derive(Debug, Serialize, Deserialize)]
struct SetCacheBehaviorCommand {
    id: u64,
    #[serde(flatten)]
    set_cache_behavior: SetCacheBehavior,
}

impl SetCacheBehaviorCommand {
    /// Constructs a new `SetCacheBehaviorCommand` with a unique ID and the provided parameters.
    fn new(params: SetCacheBehaviorParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating SetCacheBehaviorCommand with id: {}", id);
        Self {
            id,
            set_cache_behavior: SetCacheBehavior::new(params),
        }
    }
}

/// Sends a `network.setCacheBehavior` command to the WebDriver BiDi session.
pub async fn set_cache_behavior(
    session: &mut WebDriverBiDiSession,
    params: SetCacheBehaviorParameters,
) -> Result<EmptyResult, CommandError> {
    let cmd = SetCacheBehaviorCommand::new(params);
    utils::send_command(session, cmd).await
}
