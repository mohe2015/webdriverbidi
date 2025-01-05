use crate::remote::{EmptyParams, JsInt, JsUint};
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
    pub active: bool,
    #[serde(rename = "clientWindow")]
    pub client_window: ClientWindow,
    pub height: JsUint,
    pub state: ClientWindowState,
    pub width: JsUint,
    pub x: JsInt,
    pub y: JsInt,
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
    pub user_context: UserContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
    pub method: String,
    pub params: EmptyParams,
}

impl Close {
    pub fn new(params: EmptyParams) -> Self {
        Self {
            method: "browser.close".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserContext {
    pub method: String,
    pub params: EmptyParams,
}

impl CreateUserContext {
    pub fn new(params: EmptyParams) -> Self {
        Self {
            method: "browser.createUserContext".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClientWindows {
    pub method: String,
    pub params: EmptyParams,
}

impl GetClientWindows {
    pub fn new(params: EmptyParams) -> Self {
        Self {
            method: "browser.getClientWindows".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserContexts {
    pub method: String,
    pub params: EmptyParams,
}

impl GetUserContexts {
    pub fn new(params: EmptyParams) -> Self {
        Self {
            method: "browser.getUserContexts".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserContext {
    pub method: String,
    pub params: RemoveUserContextParameters,
}

impl RemoveUserContext {
    pub fn new(params: RemoveUserContextParameters) -> Self {
        Self {
            method: "browser.removeUserContext".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveUserContextParameters {
    #[serde(rename = "userContext")]
    pub user_context: UserContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClientWindowState {
    pub method: String,
    pub params: SetClientWindowStateParameters,
}

impl SetClientWindowState {
    pub fn new(params: SetClientWindowStateParameters) -> Self {
        Self {
            method: "browser.setClientWindowState".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetClientWindowStateParameters {
    #[serde(rename = "clientWindow")]
    pub client_window: ClientWindow,
    #[serde(rename = "clientWindowNamedState")]
    pub client_window_named_state: ClientWindowNamedOrRectState,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientWindowNamedOrRectState {
    ClientWindowNamedState(ClientWindowNamedState),
    ClientWindowRectState(ClientWindowRectState),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowNamedState {
    pub state: ClientWindowState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientWindowRectState {
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<JsInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<JsInt>,
}
