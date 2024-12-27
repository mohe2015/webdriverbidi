use crate::models::local::{script, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum LogEvent {
    EntryAdded(EntryAdded),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    GenericLogEntry(GenericLogEntry),
    ConsoleLogEntry(ConsoleLogEntry),
    JavascriptLogEntry(JavascriptLogEntry),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseLogEntry {
    level: Level,
    source: script::Source,
    text: Option<String>,
    timestamp: JsUint,
    #[serde(rename = "stackTrace", skip_serializing_if = "Option::is_none")]
    stack_trace: Option<script::StackTrace>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericLogEntry {
    #[serde(flatten)]
    base: BaseLogEntry,
    #[serde(rename = "type")]
    log_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsoleLogEntry {
    #[serde(flatten)]
    base: BaseLogEntry,
    #[serde(rename = "type")]
    log_type: String,
    method: String,
    args: Vec<script::RemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavascriptLogEntry {
    #[serde(flatten)]
    base: BaseLogEntry,
    #[serde(rename = "type")]
    log_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryAdded {
    method: String,
    params: Entry,
}
