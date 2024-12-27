use crate::models::remote::{EmptyParams, JsInt, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BrowserCommand {
    Close(Close),
    CreateUserContext(CreateUserContext),
    GetClientWindows(GetClientWindows),
    GetUserContexts(GetUserContexts),
    RemoveUserContext(RemoveUserContext),
    SetClientWindowState(SetClientWindowState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowInfo {
    active: bool,
    #[serde(rename = "clientWindow")]
    client_window: ClientWindow,
    height: JsUint,
    state: ClientWindowState,
    width: JsUint,
    x: JsInt,
    y: JsInt,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientWindowState {
    Fullscreen,
    Maximized,
    Minimized,
    Normal,
}

pub type ClientWindow = String;
pub type UserContext = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserContextInfo {
    #[serde(rename = "userContext")]
    user_context: UserContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
    method: String,
    params: EmptyParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserContext {
    method: String,
    params: EmptyParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClientWindows {
    method: String,
    params: EmptyParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserContexts {
    method: String,
    params: EmptyParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserContext {
    method: String,
    params: RemoveUserContextParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserContextParameters {
    #[serde(rename = "userContext")]
    user_context: UserContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClientWindowState {
    method: String,
    params: SetClientWindowStateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClientWindowStateParameters {
    #[serde(rename = "clientWindow")]
    client_window: ClientWindow,
    #[serde(rename = "clientWindowNamedState")]
    client_window_named_state: ClientWindowNamedOrRectState,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientWindowNamedOrRectState {
    ClientWindowNamedState(ClientWindowNamedState),
    ClientWindowRectState(ClientWindowRectState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowNamedState {
    state: ClientWindowState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowRectState {
    state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<JsInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<JsInt>,
}
