use crate::remote::browser::BrowserCommand;
use crate::remote::browsing_context::BrowsingContextCommand;
use crate::remote::input::InputCommand;
use crate::remote::network::NetworkCommand;
use crate::remote::script::ScriptCommand;
use crate::remote::session::SessionCommand;
use crate::remote::storage::StorageCommand;
use crate::remote::web_extension::WebExtensionCommand;
use crate::remote::Extensible;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub id: u64,
    pub command_data: CommandData,
    pub extensible: Extensible,
}

impl Command {
    pub fn new(id: u64, command_data: CommandData) -> Self {
        Self {
            id,
            command_data,
            extensible: Extensible::new(),
        }
    }
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
