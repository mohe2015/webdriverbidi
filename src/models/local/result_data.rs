use crate::local::browsing_context::BrowsingContextResult;
use crate::local::network::NetworkResult;
use crate::local::script::ScriptResult;
use crate::local::session::SessionResult;
use crate::local::storage::StorageResult;
use crate::local::web_extension::WebExtensionResult;
use crate::local::Extensible;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ResultData {
    BrowsingContextResult(BrowsingContextResult),
    EmptyResult(EmptyResult),
    NetworkResult(NetworkResult),
    ScriptResult(ScriptResult),
    SessionResult(SessionResult),
    StorageResult(StorageResult),
    WebExtensionResult(WebExtensionResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyResult {
    #[serde(flatten)]
    pub extensible: Extensible,
}
