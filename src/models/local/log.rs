use crate::local::{script, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
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
#[serde(untagged)]
pub enum Entry {
    GenericLogEntry(GenericLogEntry),
    ConsoleLogEntry(ConsoleLogEntry),
    JavascriptLogEntry(JavascriptLogEntry),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseLogEntry {
    pub level: Level,
    pub source: script::Source,
    pub text: Option<String>,
    pub timestamp: JsUint,
    #[serde(rename = "stackTrace", skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<script::StackTrace>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenericLogEntry {
    #[serde(flatten)]
    pub base: BaseLogEntry,
    #[serde(rename = "type")]
    pub log_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsoleLogEntry {
    #[serde(flatten)]
    pub base: BaseLogEntry,
    #[serde(rename = "type")]
    pub log_type: String,
    pub method: String,
    pub args: Vec<script::RemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JavascriptLogEntry {
    #[serde(flatten)]
    pub base: BaseLogEntry,
    #[serde(rename = "type")]
    pub log_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryAdded {
    pub method: String,
    pub params: Entry,
}
