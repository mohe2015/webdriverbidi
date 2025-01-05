use log::debug;
use serde::{Deserialize, Serialize};

// --------------------------------------------------

use super::id;
use super::utils;
use crate::error::CommandError;
use crate::local::result_data::EmptyResult;
use crate::local::session::*;
use crate::remote::session::*;
use crate::remote::EmptyParams;
use crate::session::WebDriverBiDiSession;

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-status

/// Represents the `session.status` command.
#[derive(Debug, Serialize, Deserialize)]
struct StatusCommand {
    id: u64,
    #[serde(flatten)]
    status: Status,
}

impl StatusCommand {
    /// Constructs a new `StatusCommand` with a unique ID and the provided parameters.
    fn new(params: EmptyParams) -> Self {
        let id = id::get_next_id();
        debug!("Creating StatusCommand with id: {}", id);
        Self {
            id,
            status: Status::new(params),
        }
    }
}

/// Sends a `session.status` command to the WebDriver BiDi session.
pub async fn status(
    session: &mut WebDriverBiDiSession,
    params: EmptyParams,
) -> Result<StatusResult, CommandError> {
    let cmd = StatusCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-new

/// Represents the `session.new` command.
#[derive(Debug, Serialize, Deserialize)]
struct NewCommand {
    id: u64,
    #[serde(flatten)]
    new: New,
}

impl NewCommand {
    /// Constructs a new `NewCommand` with a unique ID and the provided parameters.
    fn new(params: NewParameters) -> Self {
        let id = id::get_next_id();
        debug!("Creating NewCommand with id: {}", id);
        Self {
            id,
            new: New::new(params),
        }
    }
}

/// Sends a `session.new` command to the WebDriver BiDi session.
pub async fn new_session(
    session: &mut WebDriverBiDiSession,
    params: NewParameters,
) -> Result<NewResult, CommandError> {
    let cmd = NewCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-end

/// Represents the `session.end` command.
#[derive(Debug, Serialize, Deserialize)]
struct EndCommand {
    id: u64,
    #[serde(flatten)]
    end: End,
}

impl EndCommand {
    /// Constructs a new `EndCommand` with a unique ID and the provided parameters.
    fn new(params: EmptyParams) -> Self {
        let id = id::get_next_id();
        debug!("Creating EndCommand with id: {}", id);
        Self {
            id,
            end: End::new(params),
        }
    }
}

/// Sends a `session.end` command to the WebDriver BiDi session.
pub async fn end(
    session: &mut WebDriverBiDiSession,
    params: EmptyParams,
) -> Result<EmptyResult, CommandError> {
    let cmd = EndCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-subscribe

/// Represents the `session.subscribe` command.
#[derive(Debug, Serialize, Deserialize)]
struct SubscribeCommand {
    id: u64,
    #[serde(flatten)]
    subscribe: Subscribe,
}

impl SubscribeCommand {
    /// Constructs a new `SubscribeCommand` with a unique ID and the provided parameters.
    fn new(params: SubscriptionRequest) -> Self {
        let id = id::get_next_id();
        debug!("Creating SubscribeCommand with id: {}", id);
        Self {
            id,
            subscribe: Subscribe::new(params),
        }
    }
}

/// Sends a `session.subscribe` command to the WebDriver BiDi session.
pub async fn subscribe(
    session: &mut WebDriverBiDiSession,
    params: SubscriptionRequest,
) -> Result<SubscriptionRequestResult, CommandError> {
    let cmd = SubscribeCommand::new(params);
    utils::send_command(session, cmd).await
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-session-unsubscribe

/// Represents the `session.unsubscribe` command.
#[derive(Debug, Serialize, Deserialize)]
struct UnsubscribeCommand {
    id: u64,
    #[serde(flatten)]
    unsubscribe: Unsubscribe,
}

impl UnsubscribeCommand {
    /// Constructs a new `UnsubscribeCommand` with a unique ID and the provided parameters.
    fn new(params: UnsubscribeRequest) -> Self {
        let id = id::get_next_id();
        debug!("Creating UnsubscribeCommand with id: {}", id);
        Self {
            id,
            unsubscribe: Unsubscribe::new(params),
        }
    }
}

/// Sends a `session.unsubscribe` command to the WebDriver BiDi session.
pub async fn unsubscribe(
    session: &mut WebDriverBiDiSession,
    params: UnsubscribeRequest,
) -> Result<EmptyResult, CommandError> {
    let cmd = UnsubscribeCommand::new(params);
    utils::send_command(session, cmd).await
}
