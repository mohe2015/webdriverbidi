use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    CommandResponse(CommandResponse),
    ErrorResponse(ErrorResponse),
    Event(Event),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandResponse {
    #[serde(rename = "type")]
    response_type: String,
    id: JsUint,
    result: ResultData,
    #[serde(flatten)]
    extensible: Extensible,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    response_type: String,
    id: Option<JsUint>,
    error: ErrorCode,
    message: String,
    stacktrace: Option<String>,
    #[serde(flatten)]
    extensible: Extensible,
}

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
    extensible: Extensible,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: String,
    event_data: EventData,
    #[serde(flatten)]
    extensible: Extensible,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventData {
    BrowsingContextEvent(BrowsingContextEvent),
    LogEvent(LogEvent),
    NetworkEvent(NetworkEvent),
    ScriptEvent(ScriptEvent),
}

type Extensible = HashMap<String, serde_json::Value>;

// -9007199254740991..9007199254740991
type JsInt = i64;
// 0..9007199254740991
type JsUint = u64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ErrorCode {
    #[serde(rename = "invalid argument")]
    InvalidArgument,
    #[serde(rename = "invalid selector")]
    InvalidSelector,
    #[serde(rename = "invalid session id")]
    InvalidSessionId,
    #[serde(rename = "invalid web extension")]
    InvalidWebExtension,
    #[serde(rename = "move target out of bounds")]
    MoveTargetOutOfBounds,
    #[serde(rename = "no such alert")]
    NoSuchAlert,
    #[serde(rename = "no such element")]
    NoSuchElement,
    #[serde(rename = "no such frame")]
    NoSuchFrame,
    #[serde(rename = "no such handle")]
    NoSuchHandle,
    #[serde(rename = "no such history entry")]
    NoSuchHistoryEntry,
    #[serde(rename = "no such intercept")]
    NoSuchIntercept,
    #[serde(rename = "no such node")]
    NoSuchNode,
    #[serde(rename = "no such request")]
    NoSuchRequest,
    #[serde(rename = "no such script")]
    NoSuchScript,
    #[serde(rename = "no such storage partition")]
    NoSuchStoragePartition,
    #[serde(rename = "no such user context")]
    NoSuchUserContext,
    #[serde(rename = "no such web extension")]
    NoSuchWebExtension,
    #[serde(rename = "session not created")]
    SessionNotCreated,
    #[serde(rename = "unable to capture screen")]
    UnableToCaptureScreen,
    #[serde(rename = "unable to close browser")]
    UnableToCloseBrowser,
    #[serde(rename = "unable to set cookie")]
    UnableToSetCookie,
    #[serde(rename = "unable to set file input")]
    UnableToSetFileInput,
    #[serde(rename = "underspecified storage partition")]
    UnderspecifiedStoragePartition,
    #[serde(rename = "unknown command")]
    UnknownCommand,
    #[serde(rename = "unknown error")]
    UnknownError,
    #[serde(rename = "unsupported operation")]
    UnsupportedOperation,
}

// --------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub enum SessionResult {
    NewResult(session::NewResult),
    StatusResult(session::StatusResult),
}

pub mod session {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CapabilitiesRequest {
        #[serde(rename = "alwaysMatch", skip_serializing_if = "Option::is_none")]
        always_match: Option<CapabilityRequest>,
        #[serde(rename = "firstMatch", skip_serializing_if = "Option::is_none")]
        first_match: Option<Vec<CapabilityRequest>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CapabilityRequest {
        #[serde(
            rename = "acceptInsecureCerts",
            skip_serializing_if = "Option::is_none"
        )]
        accept_insecure_certs: Option<bool>,
        #[serde(rename = "browserName", skip_serializing_if = "Option::is_none")]
        browser_name: Option<String>,
        #[serde(rename = "browserVersion", skip_serializing_if = "Option::is_none")]
        browser_version: Option<String>,
        #[serde(rename = "platformName", skip_serializing_if = "Option::is_none")]
        platform_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        proxy: Option<ProxyConfiguration>,
        #[serde(
            rename = "unhandledPromptBehavior",
            skip_serializing_if = "Option::is_none"
        )]
        unhandled_prompt_behavior: Option<UserPromptHandler>,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum ProxyConfiguration {
        AutodetectProxyConfiguration(AutodetectProxyConfiguration),
        DirectProxyConfiguration(DirectProxyConfiguration),
        ManualProxyConfiguration(ManualProxyConfiguration),
        PacProxyConfiguration(PacProxyConfiguration),
        SystemProxyConfiguration(SystemProxyConfiguration),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AutodetectProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DirectProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ManualProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        #[serde(rename = "ftpProxy", skip_serializing_if = "Option::is_none")]
        ftp_proxy: Option<String>,
        #[serde(rename = "httpProxy", skip_serializing_if = "Option::is_none")]
        http_proxy: Option<String>,
        #[serde(rename = "sslProxy", skip_serializing_if = "Option::is_none")]
        ssl_proxy: Option<String>,
        #[serde(rename = "socksProxy", skip_serializing_if = "Option::is_none")]
        socks_proxy: Option<SocksProxyConfiguration>,
        #[serde(rename = "noProxy", skip_serializing_if = "Option::is_none")]
        no_proxy: Option<Vec<String>>,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SocksProxyConfiguration {
        #[serde(rename = "socksProxy")]
        socks_proxy: String,
        #[serde(rename = "socksVersion")]
        socks_version: u8,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PacProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        #[serde(rename = "proxyAutoconfigUrl")]
        proxy_autoconfig_url: String,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SystemProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserPromptHandler {
        #[serde(skip_serializing_if = "Option::is_none")]
        alert: Option<UserPromptHandlerType>,
        #[serde(rename = "beforeUnload", skip_serializing_if = "Option::is_none")]
        before_unload: Option<UserPromptHandlerType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        confirm: Option<UserPromptHandlerType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<UserPromptHandlerType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        prompt: Option<UserPromptHandlerType>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum UserPromptHandlerType {
        Accept,
        Dismiss,
        Ignore,
    }

    type Subscription = String;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct StatusResult {
        ready: bool,
        message: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NewResult {
        session_id: String,
        capabilities: Capabilities,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Capabilities {
        #[serde(rename = "acceptInsecureCerts")]
        accept_insecure_certs: bool,
        #[serde(rename = "browserName")]
        browser_name: String,
        #[serde(rename = "browserVersion")]
        browser_version: String,
        #[serde(rename = "platformName")]
        platform_name: String,
        #[serde(rename = "setWindowRect")]
        set_window_rect: bool,
        #[serde(rename = "userAgent")]
        user_agent: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        proxy: Option<ProxyConfiguration>,
        #[serde(
            rename = "unhandledPromptBehavior",
            skip_serializing_if = "Option::is_none"
        )]
        unhandled_prompt_behavior: Option<UserPromptHandler>,
        #[serde(rename = "webSocketUrl", skip_serializing_if = "Option::is_none")]
        web_socket_url: Option<String>,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SubscriptionRequestResult {
        subscription: Subscription,
    }
}

// --------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub enum BrowserResult {
    CreateUserContextResult(browser::CreateUserContextResult),
    GetUserContextsResult(browser::GetUserContextsResult),
}

pub mod browser {
    use super::*;

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
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BrowsingContextResult {
    CaptureScreenshotResult(browsing_context::CaptureScreenshotResult),
    CreateResult(browsing_context::CreateResult),
    GetTreeResult(browsing_context::GetTreeResult),
    LocateNodesResult(browsing_context::LocateNodesResult),
    NavigateResult(browsing_context::NavigateResult),
    PrintResult(browsing_context::PrintResult),
    TraverseHistoryResult(browsing_context::TraverseHistoryResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BrowsingContextEvent {
    ContextCreated(browsing_context::ContextCreated),
    ContextDestroyed(browsing_context::ContextDestroyed),
    DomContentLoaded(browsing_context::DomContentLoaded),
    DownloadWillBegin(browsing_context::DownloadWillBegin),
    FragmentNavigated(browsing_context::FragmentNavigated),
    HistoryUpdated(browsing_context::HistoryUpdated),
    Load(browsing_context::Load),
    NavigationAborted(browsing_context::NavigationAborted),
    NavigationFailed(browsing_context::NavigationFailed),
    NavigationStarted(browsing_context::NavigationStarted),
    UserPromptClosed(browsing_context::UserPromptClosed),
    UserPromptOpened(browsing_context::UserPromptOpened),
}

pub mod browsing_context {
    use super::*;

    pub type BrowsingContext = String;

    pub type InfoList = Vec<Info>;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Info {
        children: Option<InfoList>,
        #[serde(rename = "clientWindow")]
        client_window: browser::ClientWindow,
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
}

// --------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkResult {
    AddInterceptResult(network::AddInterceptResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkEvent {
    AuthRequired(network::AuthRequired),
    BeforeRequestSent(network::BeforeRequestSent),
    FetchError(network::FetchError),
    ResponseCompleted(network::ResponseCompleted),
    ResponseStarted(network::ResponseStarted),
}

pub mod network {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AuthChallenge {
        scheme: String,
        realm: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BaseParameters {
        context: Option<browsing_context::BrowsingContext>,
        is_blocked: bool,
        navigation: Option<browsing_context::Navigation>,
        redirect_count: JsUint,
        request: RequestData,
        timestamp: JsUint,
        #[serde(skip_serializing_if = "Option::is_none")]
        intercepts: Option<Vec<Intercept>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum BytesValue {
        StringValue(StringValue),
        Base64Value(Base64Value),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct StringValue {
        #[serde(rename = "type")]
        value_type: String,
        value: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Base64Value {
        #[serde(rename = "type")]
        value_type: String,
        value: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum SameSite {
        Strict,
        Lax,
        None,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Cookie {
        name: String,
        value: BytesValue,
        domain: String,
        path: String,
        size: JsUint,
        #[serde(rename = "httpOnly")]
        http_only: bool,
        secure: bool,
        #[serde(rename = "sameSite")]
        same_site: SameSite,
        #[serde(skip_serializing_if = "Option::is_none")]
        expiry: Option<JsUint>,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FetchTimingInfo {
        #[serde(rename = "timeOrigin")]
        time_origin: f64,
        #[serde(rename = "requestTime")]
        request_time: f64,
        #[serde(rename = "redirectStart")]
        redirect_start: f64,
        #[serde(rename = "redirectEnd")]
        redirect_end: f64,
        #[serde(rename = "fetchStart")]
        fetch_start: f64,
        #[serde(rename = "dnsStart")]
        dns_start: f64,
        #[serde(rename = "dnsEnd")]
        dns_end: f64,
        #[serde(rename = "connectStart")]
        connect_start: f64,
        #[serde(rename = "connectEnd")]
        connect_end: f64,
        #[serde(rename = "tlsStart")]
        tls_start: f64,
        #[serde(rename = "requestStart")]
        request_start: f64,
        #[serde(rename = "responseStart")]
        response_start: f64,
        #[serde(rename = "responseEnd")]
        response_end: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Header {
        name: String,
        value: BytesValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Initiator {
        #[serde(rename = "columnNumber", skip_serializing_if = "Option::is_none")]
        column_number: Option<JsUint>,
        #[serde(rename = "lineNumber", skip_serializing_if = "Option::is_none")]
        line_number: Option<JsUint>,
        #[serde(skip_serializing_if = "Option::is_none")]
        request: Option<Request>,
        #[serde(rename = "stackTrace", skip_serializing_if = "Option::is_none")]
        stack_trace: Option<script::StackTrace>,
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        type_: Option<InitiatorType>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum InitiatorType {
        Parser,
        Script,
        Preflight,
        Other,
    }

    pub type Intercept = String;
    pub type Request = String;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RequestData {
        request: Request,
        url: String,
        method: String,
        headers: Vec<Header>,
        cookies: Vec<Cookie>,
        #[serde(rename = "headersSize")]
        headers_size: JsUint,
        #[serde(rename = "bodySize")]
        body_size: Option<JsUint>,
        destination: String,
        #[serde(rename = "initiatorType")]
        initiator_type: Option<String>,
        timings: FetchTimingInfo,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResponseContent {
        size: JsUint,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResponseData {
        url: String,
        protocol: String,
        status: JsUint,
        #[serde(rename = "statusText")]
        status_text: String,
        #[serde(rename = "fromCache")]
        from_cache: bool,
        headers: Vec<Header>,
        #[serde(rename = "mimeType")]
        mime_type: String,
        #[serde(rename = "bytesReceived")]
        bytes_received: JsUint,
        #[serde(rename = "headersSize")]
        headers_size: Option<JsUint>,
        #[serde(rename = "bodySize")]
        body_size: Option<JsUint>,
        content: ResponseContent,
        #[serde(rename = "authChallenges", skip_serializing_if = "Option::is_none")]
        auth_challenges: Option<Vec<AuthChallenge>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AddInterceptResult {
        intercept: Intercept,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AuthRequired {
        method: String,
        params: AuthRequiredParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AuthRequiredParameters {
        #[serde(flatten)]
        pub base: BaseParameters,
        pub response: ResponseData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BeforeRequestSent {
        method: String,
        params: BeforeRequestSentParameters,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BeforeRequestSentParameters {
        #[serde(flatten)]
        pub base: BaseParameters,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub initiator: Option<Initiator>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FetchError {
        method: String,
        params: FetchErrorParameters,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FetchErrorParameters {
        #[serde(flatten)]
        pub base: BaseParameters,
        #[serde(rename = "errorText")]
        pub error_text: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResponseCompleted {
        method: String,
        params: ResponseCompletedParameters,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResponseCompletedParameters {
        #[serde(flatten)]
        pub base: BaseParameters,
        pub response: ResponseData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResponseStarted {
        method: String,
        params: ResponseStartedParameters,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResponseStartedParameters {
        #[serde(flatten)]
        pub base: BaseParameters,
        pub response: ResponseData,
    }
}

// --------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub enum ScriptResult {
    AddPreloadScriptResult(script::AddPreloadScriptResult),
    EvaluateResult(script::EvaluateResult),
    GetRealmsResult(script::GetRealmsResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ScriptEvent {
    Message(script::Message),
    RealmCreated(script::RealmCreated),
    RealmDestroyed(script::RealmDestroyed),
}

pub mod script {
    use super::*;

    pub type Channel = String;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelValue {
        #[serde(rename = "type")]
        value_type: String,
        value: ChannelProperties,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelProperties {
        channel: Channel,
        #[serde(
            rename = "serializationOptions",
            skip_serializing_if = "Option::is_none"
        )]
        serialization_options: Option<SerializationOptions>,
        #[serde(skip_serializing_if = "Option::is_none")]
        ownership: Option<ResultOwnership>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum EvaluateResult {
        EvaluateResultSuccess(EvaluateResultSuccess),
        EvaluateResultException(EvaluateResultException),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct EvaluateResultSuccess {
        #[serde(rename = "type")]
        result_type: String,
        result: RemoteValue,
        realm: Realm,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct EvaluateResultException {
        #[serde(rename = "type")]
        result_type: String,
        #[serde(rename = "exceptionDetails")]
        exception_details: ExceptionDetails,
        realm: Realm,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ExceptionDetails {
        #[serde(rename = "columnNumber")]
        column_number: JsUint,
        exception: RemoteValue,
        #[serde(rename = "lineNumber")]
        line_number: JsUint,
        #[serde(rename = "stackTrace")]
        stack_trace: StackTrace,
        text: String,
    }

    pub type Handle = String;
    pub type InternalId = String;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum LocalValue {
        RemoteReference(RemoteReference),
        PrimitiveProtocolValue(PrimitiveProtocolValue),
        ChannelValue(ChannelValue),
        ArrayLocalValue(ArrayLocalValue),
        DateLocalValue(DateLocalValue),
        MapLocalValue(MapLocalValue),
        ObjectLocalValue(ObjectLocalValue),
        RegExpLocalValue(RegExpLocalValue),
        SetLocalValue(SetLocalValue),
    }

    pub type ListLocalValue = Vec<LocalValue>;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ArrayLocalValue {
        #[serde(rename = "type")]
        value_type: String,
        value: ListLocalValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DateLocalValue {
        #[serde(rename = "type")]
        value_type: String,
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MappingLocalValue(pub Vec<(LocalValueOrText, LocalValue)>);

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum LocalValueOrText {
        LocalValue(LocalValue),
        Text(String),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MapLocalValue {
        #[serde(rename = "type")]
        value_type: String,
        value: MappingLocalValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ObjectLocalValue {
        #[serde(rename = "type")]
        value_type: String,
        value: MappingLocalValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RegExpValue {
        pattern: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        flags: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RegExpLocalValue {
        #[serde(rename = "type")]
        value_type: String,
        value: RegExpValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SetLocalValue {
        #[serde(rename = "type")]
        value_type: String,
        value: ListLocalValue,
    }

    pub type PreloadScript = String;
    pub type Realm = String;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum PrimitiveProtocolValue {
        UndefinedValue(UndefinedValue),
        NullValue(NullValue),
        StringValue(StringValue),
        NumberValue(NumberValue),
        BooleanValue(BooleanValue),
        BigIntValue(BigIntValue),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UndefinedValue {
        #[serde(rename = "type")]
        value_type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NullValue {
        #[serde(rename = "type")]
        value_type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct StringValue {
        #[serde(rename = "type")]
        value_type: String,
        value: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum SpecialNumber {
        NaN,
        #[serde(rename = "-0")]
        NegativeZero,
        Infinity,
        #[serde(rename = "-Infinity")]
        NegativeInfinity,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NumberValue {
        #[serde(rename = "type")]
        value_type: String,
        value: NumberOrSpecialNumber,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum NumberOrSpecialNumber {
        Number(f64),
        SpecialNumber(SpecialNumber),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BooleanValue {
        #[serde(rename = "type")]
        value_type: String,
        value: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BigIntValue {
        #[serde(rename = "type")]
        value_type: String,
        value: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum RealmInfo {
        WindowRealmInfo(WindowRealmInfo),
        DedicatedWorkerRealmInfo(DedicatedWorkerRealmInfo),
        SharedWorkerRealmInfo(SharedWorkerRealmInfo),
        ServiceWorkerRealmInfo(ServiceWorkerRealmInfo),
        WorkerRealmInfo(WorkerRealmInfo),
        PaintWorkletRealmInfo(PaintWorkletRealmInfo),
        AudioWorkletRealmInfo(AudioWorkletRealmInfo),
        WorkletRealmInfo(WorkletRealmInfo),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BaseRealmInfo {
        realm: Realm,
        origin: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WindowRealmInfo {
        #[serde(flatten)]
        base: BaseRealmInfo,
        #[serde(rename = "type")]
        realm_type: String,
        context: browsing_context::BrowsingContext,
        #[serde(skip_serializing_if = "Option::is_none")]
        sandbox: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DedicatedWorkerRealmInfo {
        #[serde(flatten)]
        base: BaseRealmInfo,
        #[serde(rename = "type")]
        realm_type: String,
        owners: Vec<Realm>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SharedWorkerRealmInfo {
        #[serde(flatten)]
        base: BaseRealmInfo,
        #[serde(rename = "type")]
        realm_type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ServiceWorkerRealmInfo {
        #[serde(flatten)]
        base: BaseRealmInfo,
        #[serde(rename = "type")]
        realm_type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WorkerRealmInfo {
        #[serde(flatten)]
        base: BaseRealmInfo,
        #[serde(rename = "type")]
        realm_type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PaintWorkletRealmInfo {
        #[serde(flatten)]
        base: BaseRealmInfo,
        #[serde(rename = "type")]
        realm_type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AudioWorkletRealmInfo {
        #[serde(flatten)]
        base: BaseRealmInfo,
        #[serde(rename = "type")]
        realm_type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WorkletRealmInfo {
        #[serde(flatten)]
        base: BaseRealmInfo,
        #[serde(rename = "type")]
        realm_type: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum RealmType {
        #[serde(rename = "window")]
        Window,
        #[serde(rename = "dedicated-worker")]
        DedicatedWorker,
        #[serde(rename = "shared-worker")]
        SharedWorker,
        #[serde(rename = "service-worker")]
        ServiceWorker,
        #[serde(rename = "worker")]
        Worker,
        #[serde(rename = "paint-worklet")]
        PaintWorklet,
        #[serde(rename = "audio-worklet")]
        AudioWorklet,
        #[serde(rename = "worklet")]
        Worklet,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum RemoteReference {
        SharedReference(SharedReference),
        RemoteObjectReference(RemoteObjectReference),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SharedReference {
        #[serde(rename = "sharedId")]
        shared_id: SharedId,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RemoteObjectReference {
        handle: Handle,
        #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
        shared_id: Option<SharedId>,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum RemoteValue {
        PrimitiveProtocolValue(PrimitiveProtocolValue),
        SymbolRemoteValue(SymbolRemoteValue),
        ArrayRemoteValue(ArrayRemoteValue),
        ObjectRemoteValue(ObjectRemoteValue),
        FunctionRemoteValue(FunctionRemoteValue),
        RegExpRemoteValue(RegExpRemoteValue),
        DateRemoteValue(DateRemoteValue),
        MapRemoteValue(MapRemoteValue),
        SetRemoteValue(SetRemoteValue),
        WeakMapRemoteValue(WeakMapRemoteValue),
        WeakSetRemoteValue(WeakSetRemoteValue),
        GeneratorRemoteValue(GeneratorRemoteValue),
        ErrorRemoteValue(ErrorRemoteValue),
        ProxyRemoteValue(ProxyRemoteValue),
        PromiseRemoteValue(PromiseRemoteValue),
        TypedArrayRemoteValue(TypedArrayRemoteValue),
        ArrayBufferRemoteValue(ArrayBufferRemoteValue),
        NodeListRemoteValue(NodeListRemoteValue),
        HTMLCollectionRemoteValue(HTMLCollectionRemoteValue),
        NodeRemoteValue(NodeRemoteValue),
        WindowProxyRemoteValue(WindowProxyRemoteValue),
    }

    pub type ListRemoteValue = Vec<RemoteValue>;

    pub type MappingRemoteValue = Vec<(RemoteValueOrText, RemoteValue)>;

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum RemoteValueOrText {
        RemoteValue(RemoteValue),
        Text(String),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SymbolRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ArrayRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<ListRemoteValue>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ObjectRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<MappingRemoteValue>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FunctionRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RegExpRemoteValue {
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(rename = "type")]
        value_type: String,
        value: RegExpValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DateRemoteValue {
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(rename = "type")]
        value_type: String,
        value: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MapRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<MappingRemoteValue>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SetRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<ListRemoteValue>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WeakMapRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WeakSetRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GeneratorRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ErrorRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ProxyRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PromiseRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TypedArrayRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ArrayBufferRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NodeListRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<ListRemoteValue>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct HTMLCollectionRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<ListRemoteValue>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NodeRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
        shared_id: Option<SharedId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Box<NodeProperties>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NodeProperties {
        #[serde(rename = "nodeType")]
        node_type: JsUint,
        #[serde(rename = "childNodeCount")]
        child_node_count: JsUint,
        #[serde(skip_serializing_if = "Option::is_none")]
        attributes: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        children: Option<Vec<NodeRemoteValue>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "localName")]
        local_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "mode")]
        mode: Option<NodePropertiesMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "namespaceURI")]
        namespace_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "nodeValue")]
        node_value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shadowRoot")]
        shadow_root: Option<NodeRemoteValue>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")] // Match JSON values ("open" / "closed")
    pub enum NodePropertiesMode {
        Open,
        Closed,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WindowProxyRemoteValue {
        #[serde(rename = "type")]
        value_type: String,
        value: WindowProxyProperties,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WindowProxyProperties {
        context: browsing_context::BrowsingContext,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum ResultOwnership {
        Root,
        None,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SerializationOptions {
        #[serde(rename = "maxDomDepth", skip_serializing_if = "Option::is_none")]
        max_dom_depth: Option<JsUint>,
        #[serde(rename = "maxObjectDepth", skip_serializing_if = "Option::is_none")]
        max_object_depth: Option<JsUint>,
        #[serde(rename = "includeShadowTree", skip_serializing_if = "Option::is_none")]
        include_shadow_tree: Option<IncludeShadowTree>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum IncludeShadowTree {
        None,
        Open,
        All,
    }

    pub type SharedId = String;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct StackFrame {
        #[serde(rename = "columnNumber")]
        column_number: JsUint,
        #[serde(rename = "functionName")]
        function_name: String,
        #[serde(rename = "lineNumber")]
        line_number: JsUint,
        url: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct StackTrace {
        #[serde(rename = "callFrames")]
        call_frames: Vec<StackFrame>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Source {
        realm: Realm,
        #[serde(skip_serializing_if = "Option::is_none")]
        context: Option<browsing_context::BrowsingContext>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AddPreloadScriptResult {
        script: PreloadScript,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GetRealmsResult {
        realms: Vec<RealmInfo>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        method: String,
        params: MessageParameters,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MessageParameters {
        channel: Channel,
        data: RemoteValue,
        source: Source,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RealmCreated {
        method: String,
        params: RealmInfo,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RealmDestroyed {
        method: String,
        params: RealmDestroyedParameters,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RealmDestroyedParameters {
        realm: Realm,
    }
}

// --------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub enum StorageResult {
    DeleteCookiesResult(storage::DeleteCookiesResult),
    GetCookiesResult(storage::GetCookiesResult),
    SetCookieResult(storage::SetCookieResult),
}

pub mod storage {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PartionKey {
        #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
        user_context: Option<String>,
        #[serde(rename = "sourceOrigin", skip_serializing_if = "Option::is_none")]
        source_origin: Option<String>,
        #[serde(flatten)]
        extensible: Extensible,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GetCookiesResult {
        cookies: Vec<network::Cookie>,
        #[serde(rename = "partitionKey")]
        partition_key: PartionKey,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SetCookieResult {
        #[serde(rename = "partitionKey")]
        partition_key: PartionKey,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DeleteCookiesResult {
        #[serde(rename = "partitionKey")]
        partition_key: PartionKey,
    }
}

// --------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub enum LogEvent {
    EntryAdded(log::EntryAdded),
}

pub mod log {
    use super::*;

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
}

// --------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub enum WebExtensionResult {
    InstallResult(web_extension::InstallResult),
}

pub mod web_extension {
    use super::*;

    pub type Extension = String;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct InstallResult {
        extension: Extension,
    }
}
