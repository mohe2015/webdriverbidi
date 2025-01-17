use crate::local::{browser, script, session, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BrowsingContextResult {
    CaptureScreenshotResult(CaptureScreenshotResult),
    CreateResult(CreateResult),
    GetTreeResult(GetTreeResult),
    LocateNodesResult(LocateNodesResult),
    NavigateResult(NavigateResult),
    PrintResult(PrintResult),
    TraverseHistoryResult(TraverseHistoryResult),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BrowsingContextEvent {
    ContextCreated(ContextCreated),
    ContextDestroyed(ContextDestroyed),
    DomContentLoaded(DomContentLoaded),
    DownloadWillBegin(DownloadWillBegin),
    FragmentNavigated(FragmentNavigated),
    HistoryUpdated(HistoryUpdated),
    Load(Load),
    NavigationAborted(NavigationAborted),
    NavigationFailed(NavigationFailed),
    NavigationStarted(NavigationStarted),
    UserPromptClosed(UserPromptClosed),
    UserPromptOpened(UserPromptOpened),
}

pub type BrowsingContext = String;

pub type InfoList = Vec<Info>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub children: Option<InfoList>,
    #[serde(rename = "clientWindow", skip_serializing_if = "Option::is_none")]
    pub client_window: Option<browser::ClientWindow>,
    pub context: BrowsingContext,
    #[serde(rename = "originalOpener")]
    pub original_opener: Option<BrowsingContext>,
    pub url: String,
    #[serde(rename = "userContext")]
    pub user_context: browser::UserContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<BrowsingContext>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Locator {
    AccessibilityLocator(AccessibilityLocator),
    CssLocator(CssLocator),
    InnerTextLocator(InnerTextLocator),
    XPathLocator(XPathLocator),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityLocator {
    #[serde(rename = "type")]
    pub locator_type: String,
    pub value: AccessibilityLocatorValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityLocatorValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CssLocator {
    #[serde(rename = "type")]
    pub locator_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnerTextLocator {
    #[serde(rename = "type")]
    pub locator_type: String,
    pub value: String,
    #[serde(rename = "ignoreCase", skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[serde(rename = "matchType", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<InnerTextLocatorMatchType>,
    #[serde(rename = "maxDepth", skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<JsUint>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum InnerTextLocatorMatchType {
    Full,
    Partial,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XPathLocator {
    #[serde(rename = "type")]
    pub locator_type: String,
    pub value: String,
}

pub type Navigation = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationInfo {
    pub context: BrowsingContext,
    pub navigation: Option<Navigation>,
    pub timestamp: JsUint,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum UserPromptType {
    Alert,
    BeforeUnload,
    Confirm,
    Prompt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptureScreenshotResult {
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResult {
    pub context: BrowsingContext,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTreeResult {
    pub contexts: InfoList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocateNodesResult {
    pub nodes: Vec<script::NodeRemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigateResult {
    pub navigation: Option<Navigation>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrintResult {
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TraverseHistoryResult {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextCreated {
    pub method: String,
    pub params: Info,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextDestroyed {
    pub method: String,
    pub params: Info,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationStarted {
    pub method: String,
    pub params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FragmentNavigated {
    pub method: String,
    pub params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryUpdated {
    pub method: String,
    pub params: HistoryUpdatedParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryUpdatedParameters {
    pub context: BrowsingContext,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomContentLoaded {
    pub method: String,
    pub params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Load {
    pub method: String,
    pub params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadWillBegin {
    pub method: String,
    pub params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationAborted {
    pub method: String,
    pub params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationFailed {
    pub method: String,
    pub params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPromptClosed {
    pub method: String,
    pub params: UserPromptClosedParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPromptClosedParameters {
    pub context: BrowsingContext,
    pub accepted: bool,
    #[serde(rename = "type")]
    pub prompt_type: UserPromptType,
    #[serde(rename = "userText", skip_serializing_if = "Option::is_none")]
    pub user_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPromptOpened {
    pub method: String,
    pub params: UserPromptOpenedParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPromptOpenedParameters {
    pub context: BrowsingContext,
    pub handler: session::UserPromptHandlerType,
    pub message: String,
    #[serde(rename = "type")]
    pub prompt_type: UserPromptType,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}
