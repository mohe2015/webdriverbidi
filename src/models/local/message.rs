use crate::models::local::command_response::CommandResponse;
use crate::models::local::error_response::ErrorResponse;
use crate::models::local::event::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    CommandResponse(CommandResponse),
    ErrorResponse(ErrorResponse),
    Event(Event),
}
