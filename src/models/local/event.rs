use crate::local::browsing_context::BrowsingContextEvent;
use crate::local::log::LogEvent;
use crate::local::network::NetworkEvent;
use crate::local::script::ScriptEvent;
use crate::local::Extensible;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    pub event_data: EventData,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventData {
    BrowsingContextEvent(BrowsingContextEvent),
    LogEvent(LogEvent),
    NetworkEvent(NetworkEvent),
    ScriptEvent(ScriptEvent),
}
