use crate::models::local::{JsInt, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum BrowserResult {
    CreateUserContextResult(CreateUserContextResult),
    GetUserContextsResult(GetUserContextsResult),
}

pub type ClientWindow = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientWindowInfo {
    active: bool,
    #[serde(rename = "clientWindow")]
    client_window: ClientWindow,
    height: JsUint,
    state: ClientWindowInfoState,
    width: JsUint,
    x: JsInt,
    y: JsInt,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ClientWindowInfoState {
    Fullscreen,
    Maximized,
    Minimized,
    Normal,
}

pub type UserContext = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserContextInfo {
    #[serde(rename = "userContext")]
    user_context: UserContext,
}

pub type CreateUserContextResult = UserContextInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetClientWindowsResult {
    #[serde(rename = "clientWindows")]
    client_windows: Vec<ClientWindowInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserContextsResult {
    #[serde(rename = "userContexts")]
    user_contexts: Vec<UserContextInfo>,
}
