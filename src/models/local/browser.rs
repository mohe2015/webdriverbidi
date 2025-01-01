use crate::local::{JsInt, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum BrowserResult {
    CreateUserContextResult(CreateUserContextResult),
    GetUserContextsResult(GetUserContextsResult),
}

pub type ClientWindow = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientWindowInfo {
    pub active: bool,
    #[serde(rename = "clientWindow")]
    pub client_window: ClientWindow,
    pub height: JsUint,
    pub state: ClientWindowInfoState,
    pub width: JsUint,
    pub x: JsInt,
    pub y: JsInt,
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
    pub user_context: UserContext,
}

pub type CreateUserContextResult = UserContextInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetClientWindowsResult {
    #[serde(rename = "clientWindows")]
    pub client_windows: Vec<ClientWindowInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserContextsResult {
    #[serde(rename = "userContexts")]
    pub user_contexts: Vec<UserContextInfo>,
}
