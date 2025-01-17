use crate::local::command_response::CommandResponse;
use crate::local::error_response::ErrorResponse;
use crate::local::event::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    CommandResponse(CommandResponse),
    ErrorResponse(ErrorResponse),
    Event(Event),
}
