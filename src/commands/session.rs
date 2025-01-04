use log::debug;
use serde::{Deserialize, Serialize};

// --------------------------------------------------

use super::id;
use super::utils;
use crate::error::CommandError;
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
            status: crate::remote::session::Status::new(params),
        }
    }
}

/// Sends a `session.status` command to the WebDriver BiDi session.
pub async fn status(
    session: &mut WebDriverBiDiSession,
    params: EmptyParams,
) -> Result<StatusResult, CommandError> {
    let status_cmd = StatusCommand::new(params);
    utils::send_command(session, status_cmd).await
}

// --------------------------------------------------
