use crate::models::local::browsing_context::BrowsingContextEvent;
use crate::models::local::log::LogEvent;
use crate::models::local::network::NetworkEvent;
use crate::models::local::script::ScriptEvent;
use crate::models::local::Extensible;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: String,
    event_data: EventData,
    #[serde(flatten)]
    extensible: Extensible,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventData {
    BrowsingContextEvent(BrowsingContextEvent),
    LogEvent(LogEvent),
    NetworkEvent(NetworkEvent),
    ScriptEvent(ScriptEvent),
}
