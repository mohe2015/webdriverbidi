use log::{debug, error};
use serde::{Deserialize, Serialize};

// --------------------------------------------------

use super::id;
use crate::error::CommandError;
use crate::session::WebDriverBiDiSession;

/// Sends a command to the WebDriver BiDi session and processes the result.
pub async fn send_command<C, R>(
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
