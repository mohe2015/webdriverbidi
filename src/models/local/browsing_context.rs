use crate::models::local::{browser, script, session, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    children: Option<InfoList>,
    #[serde(rename = "clientWindow", skip_serializing_if = "Option::is_none")]
    client_window: Option<browser::ClientWindow>,
    context: BrowsingContext,
    #[serde(rename = "originalOpener")]
    original_opener: Option<BrowsingContext>,
    url: String,
    #[serde(rename = "userContext")]
    user_context: browser::UserContext,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<BrowsingContext>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Locator {
    AccessibilityLocator(AccessibilityLocator),
    CssLocator(CssLocator),
    InnerTextLocator(InnerTextLocator),
    XPathLocator(XPathLocator),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityLocator {
    #[serde(rename = "type")]
    locator_type: String,
    value: AccessibilityLocatorValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessibilityLocatorValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CssLocator {
    #[serde(rename = "type")]
    locator_type: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InnerTextLocator {
    #[serde(rename = "type")]
    locator_type: String,
    value: String,
    #[serde(rename = "ignoreCase", skip_serializing_if = "Option::is_none")]
    ignore_case: Option<bool>,
    #[serde(rename = "matchType", skip_serializing_if = "Option::is_none")]
    match_type: Option<InnerTextLocatorMatchType>,
    #[serde(rename = "maxDepth", skip_serializing_if = "Option::is_none")]
    max_depth: Option<JsUint>,
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
    locator_type: String,
    value: String,
}

pub type Navigation = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationInfo {
    context: BrowsingContext,
    navigation: Option<Navigation>,
    timestamp: JsUint,
    url: String,
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
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResult {
    context: BrowsingContext,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTreeResult {
    contexts: InfoList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocateNodesResult {
    nodes: Vec<script::NodeRemoteValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigateResult {
    navigation: Option<Navigation>,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrintResult {
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TraverseHistoryResult {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextCreated {
    method: String,
    params: Info,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextDestroyed {
    method: String,
    params: Info,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationStarted {
    method: String,
    params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FragmentNavigated {
    method: String,
    params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryUpdated {
    method: String,
    params: HistoryUpdatedParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryUpdatedParameters {
    context: BrowsingContext,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomContentLoaded {
    method: String,
    params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Load {
    method: String,
    params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadWillBegin {
    method: String,
    params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationAborted {
    method: String,
    params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavigationFailed {
    method: String,
    params: NavigationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPromptClosed {
    method: String,
    params: UserPromptClosedParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPromptClosedParameters {
    context: BrowsingContext,
    accepted: bool,
    #[serde(rename = "type")]
    prompt_type: UserPromptType,
    #[serde(rename = "userText", skip_serializing_if = "Option::is_none")]
    user_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPromptOpened {
    method: String,
    params: UserPromptOpenedParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPromptOpenedParameters {
    context: BrowsingContext,
    handler: session::UserPromptHandlerType,
    message: String,
    #[serde(rename = "type")]
    prompt_type: UserPromptType,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    default_value: Option<String>,
}
