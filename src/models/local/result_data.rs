use crate::models::local::browsing_context::BrowsingContextResult;
use crate::models::local::network::NetworkResult;
use crate::models::local::script::ScriptResult;
use crate::models::local::session::SessionResult;
use crate::models::local::storage::StorageResult;
use crate::models::local::web_extension::WebExtensionResult;
use crate::models::local::Extensible;
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
