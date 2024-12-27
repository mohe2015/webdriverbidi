use crate::models::remote::browser::BrowserCommand;
use crate::models::remote::browsing_context::BrowsingContextCommand;
use crate::models::remote::input::InputCommand;
use crate::models::remote::network::NetworkCommand;
use crate::models::remote::script::ScriptCommand;
use crate::models::remote::session::SessionCommand;
use crate::models::remote::storage::StorageCommand;
use crate::models::remote::web_extension::WebExtensionCommand;
use crate::models::remote::Extensible;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    id: u64,
    command_data: CommandData,
    extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandData {
    BrowserCommand(BrowserCommand),
    BrowsingContextCommand(BrowsingContextCommand),
    InputCommand(InputCommand),
    NetworkCommand(NetworkCommand),
    ScriptCommand(ScriptCommand),
    SessionCommand(SessionCommand),
    StorageCommand(StorageCommand),
    WebExtensionCommand(WebExtensionCommand),
}
