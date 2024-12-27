use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    id: u64,
    command_data: CommandData,
    extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandData {
    BrowserCommand(BrowserCommand),
    BrowsingContextCommand(BrowsingContextCommand),
    InputCommand(InputCommand),
    NetworkCommand(NetworkCommand),
    ScriptCommand(ScriptCommand),
    SessionCommand(SessionCommand),
    StorageCommand(StorageCommand),
    WebExtensionCommand(WebExtensionCommand),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyParams {
    extensible: Extensible,
}

pub type Extensible = HashMap<String, String>;

// -9007199254740991..9007199254740991
pub type JsInt = i64;
// 0..9007199254740991
pub type JsUint = u64;

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionCommand {
    End(session::End),
    New(session::New),
    Status(session::Status),
    Subscribe(session::Subscribe),
    Unsubscribe(session::Unsubscribe),
}

pub mod session {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CapabilitiesRequest {
        #[serde(rename = "alwaysMatch", skip_serializing_if = "Option::is_none")]
        always_match: Option<CapabilityRequest>,
        #[serde(rename = "firstMatch", skip_serializing_if = "Option::is_none")]
        first_match: Option<Vec<CapabilityRequest>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
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
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ProxyConfiguration {
        AutodetectProxyConfiguration(AutodetectProxyConfiguration),
        DirectProxyConfiguration(DirectProxyConfiguration),
        ManualProxyConfiguration(ManualProxyConfiguration),
        PacProxyConfiguration(PacProxyConfiguration),
        SystemProxyConfiguration(SystemProxyConfiguration),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutodetectProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DirectProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ManualProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        #[serde(rename = "ftpProxy", skip_serializing_if = "Option::is_none")]
        ftp_proxy: Option<String>,
        #[serde(rename = "httpProxy", skip_serializing_if = "Option::is_none")]
        http_proxy: Option<String>,
        #[serde(rename = "sslProxy", skip_serializing_if = "Option::is_none")]
        ssl_proxy: Option<String>,
        #[serde(
            rename = "socksProxyConfiguration",
            skip_serializing_if = "Option::is_none"
        )]
        socks_proxy_configuration: Option<SocksProxyConfiguration>,
        #[serde(rename = "noProxy", skip_serializing_if = "Option::is_none")]
        no_proxy: Option<Vec<String>>,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SocksProxyConfiguration {
        #[serde(rename = "socksProxy")]
        socks_proxy: String,
        #[serde(rename = "socksVersion")]
        socks_version: u8,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PacProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        #[serde(rename = "proxyAutoconfigUrl")]
        proxy_autoconfig_url: String,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SystemProxyConfiguration {
        #[serde(rename = "proxyType")]
        proxy_type: String,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum UserPromptHandlerType {
        Accept,
        Dismiss,
        Ignore,
    }

    pub type Subscription = String;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubscriptionRequest {
        events: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        contexts: Option<Vec<browsing_context::BrowsingContext>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UnsubscribeByIDRequest {
        subscriptions: Vec<Subscription>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UnsubscribeByAttributesRequest {
        events: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        contexts: Option<Vec<browsing_context::BrowsingContext>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Status {
        method: String,
        params: EmptyParams,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct New {
        method: String,
        params: NewParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NewParameters {
        capabilities: CapabilitiesRequest,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct End {
        method: String,
        params: EmptyParams,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Subscribe {
        method: String,
        params: SubscriptionRequest,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Unsubscribe {
        method: String,
        params: UnsubscribeRequest,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum UnsubscribeRequest {
        UnsubscribeByAttributesRequest(UnsubscribeByAttributesRequest),
        UnsubscribeByIDRequest(UnsubscribeByIDRequest),
    }
}

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub enum BrowserCommand {
    Close(browser::Close),
    CreateUserContext(browser::CreateUserContext),
    GetClientWindows(browser::GetClientWindows),
    GetUserContexts(browser::GetUserContexts),
    RemoveUserContext(browser::RemoveUserContext),
    SetClientWindowState(browser::SetClientWindowState),
}

pub mod browser {
    use super::*;

    pub type ClientWindow = String;

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
}

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub enum BrowsingContextCommand {
    Activate(browsing_context::Activate),
    CaptureScreenshot(browsing_context::CaptureScreenshot),
    Close(browsing_context::Close),
    Create(browsing_context::Create),
    GetTree(browsing_context::GetTree),
    HandleUserPrompt(browsing_context::HandleUserPrompt),
    LocateNodes(browsing_context::LocateNodes),
    Navigate(browsing_context::Navigate),
    Print(browsing_context::Print),
    Reload(browsing_context::Reload),
    SetViewport(browsing_context::SetViewport),
    TraverseHistory(browsing_context::TraverseHistory),
}

pub mod browsing_context {
    use super::*;

    pub type BrowsingContext = String;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Locator {
        AccessibilityLocator(AccessibilityLocator),
        CssLocator(CssLocator),
        InnerTextLocator(InnerTextLocator),
        XPathLocator(XPathLocator),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccessibilityLocator {
        #[serde(rename = "type")]
        locator_type: String,
        value: AccessibilityLocatorValue,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AccessibilityLocatorValue {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        role: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CssLocator {
        #[serde(rename = "type")]
        locator_type: String,
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum InnerTextLocatorMatchType {
        Full,
        Partial,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct XPathLocator {
        #[serde(rename = "type")]
        locator_type: String,
        value: String,
    }

    // pub type Navigation = String;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum ReadinessState {
        Complete,
        Interactive,
        None,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum UserPromptType {
        Alert,
        BeforeUnload,
        Confirm,
        Prompt,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Activate {
        method: String,
        params: ActivateParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ActivateParameters {
        context: BrowsingContext,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CaptureScreenshot {
        method: String,
        params: CaptureScreenshotParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CaptureScreenshotParameters {
        context: BrowsingContext,
        #[serde(skip_serializing_if = "Option::is_none")]
        origin: Option<CaptureScreenshotParametersOrigin>,
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<ImageFormat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        clip: Option<ClipRectangle>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum CaptureScreenshotParametersOrigin {
        Document,
        Viewport,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImageFormat {
        #[serde(rename = "type")]
        image_format_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        quality: Option<f32>, // 0.0..1.0
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ClipRectangle {
        BoxClipRectangle(BoxClipRectangle),
        ElementClipRectangle(ElementClipRectangle),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElementClipRectangle {
        #[serde(rename = "type")]
        clip_rectangle_type: String,
        element: script::SharedReference,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BoxClipRectangle {
        #[serde(rename = "type")]
        clip_rectangle_type: String,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Close {
        method: String,
        params: CloseParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CloseParameters {
        context: BrowsingContext,
        #[serde(rename = "promptUnload", skip_serializing_if = "Option::is_none")]
        prompt_unload: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Create {
        method: String,
        params: CreateParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum CreateType {
        Tab,
        Window,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateParameters {
        #[serde(rename = "type")]
        create_type: CreateType,
        #[serde(rename = "referenceContext", skip_serializing_if = "Option::is_none")]
        reference_context: Option<BrowsingContext>,
        #[serde(skip_serializing_if = "Option::is_none")]
        background: Option<bool>,
        #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
        user_context: Option<browser::UserContext>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTree {
        method: String,
        params: GetTreeParameters,
    }
    
    impl GetTree {
        pub fn new(params: GetTreeParameters) -> Self {
            Self {
                method: "browsingContext.getTree".to_string(),
                params,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetTreeParameters {
        #[serde(rename = "maxDepth", skip_serializing_if = "Option::is_none")]
        pub max_depth: Option<JsUint>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub root: Option<BrowsingContext>,
    }
    
    impl GetTreeParameters {
        pub fn new() -> Self {
            Self {
                max_depth: None,
                root: None,
            }
        }
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HandleUserPrompt {
        method: String,
        params: HandleUserPromptParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct HandleUserPromptParameters {
        context: BrowsingContext,
        #[serde(skip_serializing_if = "Option::is_none")]
        accept: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        user_text: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LocateNodes {
        method: String,
        params: LocateNodesParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LocateNodesParameters {
        context: BrowsingContext,
        locator: Locator,
        #[serde(rename = "maxNodeCount", skip_serializing_if = "Option::is_none")]
        max_node_count: Option<JsUint>,
        #[serde(
            rename = "serializationOptions",
            skip_serializing_if = "Option::is_none"
        )]
        serialization_options: Option<script::SerializationOptions>,
        #[serde(rename = "startNodes", skip_serializing_if = "Option::is_none")]
        start_nodes: Option<Vec<script::SharedReference>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Navigate {
        pub method: String, // "browsingContext.navigate"
        pub params: NavigateParameters,
    }

    impl Navigate {
        pub fn new(params: NavigateParameters) -> Self {
            Self {
                method: "browsingContext.navigate".to_string(),
                params,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NavigateParameters {
        pub context: BrowsingContext,
        pub url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub wait: Option<ReadinessState>,
    }

    impl NavigateParameters {
        pub fn new(context: BrowsingContext, url: String, wait: Option<ReadinessState>) -> Self {
            Self { context, url, wait }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Print {
        method: String,
        params: PrintParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PrintParameters {
        context: BrowsingContext,
        #[serde(skip_serializing_if = "Option::is_none")]
        background: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        margin: Option<PrintMarginParameters>,
        #[serde(skip_serializing_if = "Option::is_none")]
        orientation: Option<PrintParametersOrientation>,
        #[serde(skip_serializing_if = "Option::is_none")]
        page: Option<PrintPageParameters>,
        #[serde(rename = "pageRanges", skip_serializing_if = "Option::is_none")]
        page_ranges: Option<Vec<JsUintOrText>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        scale: Option<f32>, // 0.1..2.0
        #[serde(rename = "shrinkToFit", skip_serializing_if = "Option::is_none")]
        shrink_to_fit: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum PrintParametersOrientation {
        Landscape,
        Portrait,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum JsUintOrText {
        JsUint(JsUint),
        Text(String),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PrintMarginParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        bottom: Option<f32>, // 0.0..
        #[serde(skip_serializing_if = "Option::is_none")]
        left: Option<f32>, // 0.0..
        #[serde(skip_serializing_if = "Option::is_none")]
        right: Option<f32>, // 0.0..
        #[serde(skip_serializing_if = "Option::is_none")]
        top: Option<f32>, // 0.0..
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PrintPageParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<f32>, // 0.0352..
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<f32>, // 0.0352..
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Reload {
        method: String,
        params: ReloadParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReloadParameters {
        context: BrowsingContext,
        #[serde(rename = "ignoreCache", skip_serializing_if = "Option::is_none")]
        ignore_cache: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        wait: Option<ReadinessState>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetViewport {
        method: String,
        params: SetViewportParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetViewportParameters {
        context: BrowsingContext,
        #[serde(skip_serializing_if = "Option::is_none")]
        viewport: Option<Viewport>,
        #[serde(rename = "devicePixelRatio", skip_serializing_if = "Option::is_none")]
        device_pixel_ratio: Option<f32>, // 0.0..
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Viewport {
        width: JsUint,
        height: JsUint,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TraverseHistory {
        method: String,
        params: TraverseHistoryParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TraverseHistoryParameters {
        context: BrowsingContext,
        delta: JsInt,
    }
}

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkCommand {
    AddIntercept(network::AddIntercept),
    ContinueRequest(network::ContinueRequest),
    ContinueResponse(network::ContinueResponse),
    ContinueWithAuth(network::ContinueWithAuth),
    FailRequest(network::FailRequest),
    ProvideResponse(network::ProvideResponse),
    RemoveIntercept(network::RemoveIntercept),
    SetCacheBehavior(network::SetCacheBehavior),
}

pub mod network {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AuthCredentials {
        #[serde(rename = "type")]
        auth_credentials_type: String,
        username: String,
        password: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum BytesValue {
        StringValue(StringValue),
        Base64Value(Base64Value),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StringValue {
        #[serde(rename = "type")]
        string_value_type: String,
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Base64Value {
        #[serde(rename = "type")]
        base64_value_type: String,
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum SameSite {
        Strict,
        Lax,
        None,
    }

    #[derive(Debug, Serialize, Deserialize)]
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
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CookieHeader {
        name: String,
        value: BytesValue,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Header {
        name: String,
        value: BytesValue,
    }

    pub type Intercept = String;

    pub type Request = String;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetCookieHeader {
        name: String,
        value: BytesValue,
        #[serde(skip_serializing_if = "Option::is_none")]
        domain: Option<String>,
        #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
        http_only: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        expiry: Option<String>,
        #[serde(rename = "maxAge", skip_serializing_if = "Option::is_none")]
        max_age: Option<JsInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
        #[serde(rename = "sameSite", skip_serializing_if = "Option::is_none")]
        same_site: Option<SameSite>,
        #[serde(skip_serializing_if = "Option::is_none")]
        secure: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum UrlPattern {
        UrlPatternPattern(UrlPatternPattern),
        UrlPatternString(UrlPatternString),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UrlPatternPattern {
        #[serde(rename = "type")]
        url_pattern_pattern_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        protocol: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        hostname: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        port: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pathname: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        search: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UrlPatternString {
        #[serde(rename = "type")]
        url_pattern_string_type: String,
        pattern: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AddIntercept {
        method: String,
        params: AddInterceptParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AddInterceptParameters {
        phases: Vec<InterceptPhase>,
        #[serde(skip_serializing_if = "Option::is_none")]
        contexts: Option<Vec<browsing_context::BrowsingContext>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        url_patterns: Option<Vec<UrlPattern>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum InterceptPhase {
        #[serde(rename = "beforeRequestSent")]
        BeforeRequestSent,
        #[serde(rename = "responseStarted")]
        ResponseStarted,
        #[serde(rename = "authRequired")]
        AuthRequired,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContinueRequest {
        method: String,
        params: ContinueRequestParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContinueRequestParameters {
        request: Request,
        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<BytesValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cookies: Option<Vec<CookieHeader>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<Vec<Header>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        method: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContinueResponse {
        method: String,
        params: ContinueResponseParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContinueResponseParameters {
        request: Request,
        #[serde(skip_serializing_if = "Option::is_none")]
        cookies: Option<Vec<SetCookieHeader>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        credentials: Option<AuthCredentials>,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<Vec<Header>>,
        #[serde(rename = "reasonPhrase", skip_serializing_if = "Option::is_none")]
        reason_phrase: Option<String>,
        #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
        status_code: Option<JsUint>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContinueWithAuth {
        method: String,
        params: ContinueWithAuthParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContinueWithAuthParameters {
        request: Request,
        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        auth_option: Option<ContinueWithAuthOption>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ContinueWithAuthOption {
        Credentials(ContinueWithAuthCredentials),
        NoCredentials(ContinueWithAuthNoCredentials),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContinueWithAuthCredentials {
        action: String,
        credentials: AuthCredentials,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContinueWithAuthNoCredentials {
        action: NoCredentialsAction,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum NoCredentialsAction {
        Default,
        Cancel,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FailRequest {
        method: String,
        params: FailRequestParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FailRequestParameters {
        request: Request,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProvideResponse {
        method: String,
        params: ProvideResponseParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProvideResponseParameters {
        request: Request,
        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<BytesValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cookies: Option<Vec<SetCookieHeader>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<Vec<Header>>,
        #[serde(rename = "reasonPhrase", skip_serializing_if = "Option::is_none")]
        reason_phrase: Option<String>,
        #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
        status_code: Option<JsUint>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RemoveIntercept {
        method: String,
        params: RemoveInterceptParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RemoveInterceptParameters {
        intercept: Intercept,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetCacheBehavior {
        method: String,
        params: SetCacheBehaviorParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetCacheBehaviorParameters {
        cache_behavior: CacheBehavior,
        #[serde(skip_serializing_if = "Option::is_none")]
        contexts: Option<Vec<browsing_context::BrowsingContext>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum CacheBehavior {
        Default,
        Bypass,
    }
}

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub enum ScriptCommand {
    AddPreloadScript(script::AddPreloadScript),
    CallFunction(script::CallFunction),
    Disown(script::Disown),
    Evaluate(script::Evaluate),
    GetRealms(script::GetRealms),
    RemovePreloadScript(script::RemovePreloadScript),
}

pub mod script {
    use super::*;

    pub type Channel = String;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChannelValue {
        #[serde(rename = "type")]
        channel_value_type: String,
        value: ChannelProperties,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    pub enum EvaluateResult {
        EvaluateResultSuccess(EvaluateResultSuccess),
        EvaluateResultException(EvaluateResultException),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct EvaluateResultSuccess {
        #[serde(rename = "type")]
        evaluate_result_success_type: String,
        result: RemoteValue,
        realm: Realm,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct EvaluateResultException {
        #[serde(rename = "type")]
        evaluate_result_exception_type: String,
        #[serde(rename = "exceptionDetails")]
        exception_details: ExceptionDetails,
        realm: Realm,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ArrayLocalValue {
        #[serde(rename = "type")]
        array_local_value_type: String,
        value: ListLocalValue,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DateLocalValue {
        #[serde(rename = "type")]
        date_local_value_type: String,
        value: String,
    }

    pub type MappingLocalValue = Vec<(LocalValueOrText, LocalValue)>;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum LocalValueOrText {
        LocalValue(LocalValue),
        Text(String),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MapLocalValue {
        #[serde(rename = "type")]
        map_local_value_type: String,
        value: MappingLocalValue,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ObjectLocalValue {
        #[serde(rename = "type")]
        object_local_value_type: String,
        value: MappingLocalValue,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RegExpValue {
        pattern: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        flags: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RegExpLocalValue {
        #[serde(rename = "type")]
        regexp_local_value_type: String,
        value: RegExpValue,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetLocalValue {
        #[serde(rename = "type")]
        set_local_value_type: String,
        value: ListLocalValue,
    }

    pub type PreloadScript = String;

    pub type Realm = String;

    #[derive(Debug, Serialize, Deserialize)]
    pub enum PrimitiveProtocolValue {
        UndefinedValue(UndefinedValue),
        NullValue(NullValue),
        StringValue(StringValue),
        NumberValue(NumberValue),
        BooleanValue(BooleanValue),
        BigIntValue(BigIntValue),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UndefinedValue {
        #[serde(rename = "type")]
        undefined_value_type: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NullValue {
        #[serde(rename = "type")]
        null_value_type: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StringValue {
        #[serde(rename = "type")]
        string_value_type: String,
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum SpecialNumber {
        NaN,
        #[serde(rename = "-0")]
        NegativeZero,
        Infinity,
        #[serde(rename = "-Infinity")]
        NegativeInfinity,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NumberValue {
        #[serde(rename = "type")]
        number_value_type: String,
        value: NumberOrSpecialNumber,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum NumberOrSpecialNumber {
        Number(f64),
        SpecialNumber(SpecialNumber),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BooleanValue {
        #[serde(rename = "type")]
        boolean_value_type: String,
        value: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BigIntValue {
        #[serde(rename = "type")]
        bigint_value_type: String,
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    pub enum RemoteReference {
        SharedReference(SharedReference),
        RemoteObjectReference(RemoteObjectReference),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SharedReference {
        #[serde(rename = "sharedId")]
        shared_id: SharedId,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RemoteObjectReference {
        handle: Handle,
        #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
        shared_id: Option<SharedId>,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    pub enum RemoteValueOrText {
        RemoteValue(RemoteValue),
        Text(String),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SymbolRemoteValue {
        #[serde(rename = "type")]
        symbol_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ArrayRemoteValue {
        #[serde(rename = "type")]
        array_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<ListRemoteValue>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ObjectRemoteValue {
        #[serde(rename = "type")]
        object_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<MappingRemoteValue>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct FunctionRemoteValue {
        #[serde(rename = "type")]
        function_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RegExpRemoteValue {
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(flatten)]
        reg_exp_local_value: RegExpLocalValue,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DateRemoteValue {
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(flatten)]
        date_local_value: DateLocalValue,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct MapRemoteValue {
        #[serde(rename = "type")]
        map_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        value: Option<MappingRemoteValue>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetRemoteValue {
        #[serde(rename = "type")]
        set_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        value: Option<ListRemoteValue>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct WeakMapRemoteValue {
        #[serde(rename = "type")]
        weak_map_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct WeakSetRemoteValue {
        #[serde(rename = "type")]
        weak_set_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GeneratorRemoteValue {
        #[serde(rename = "type")]
        generator_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ErrorRemoteValue {
        #[serde(rename = "type")]
        error_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProxyRemoteValue {
        #[serde(rename = "type")]
        proxy_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PromiseRemoteValue {
        #[serde(rename = "type")]
        promise_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TypedArrayRemoteValue {
        #[serde(rename = "type")]
        typed_array_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ArrayBufferRemoteValue {
        #[serde(rename = "type")]
        array_buffer_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NodeListRemoteValue {
        #[serde(rename = "type")]
        node_list_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<ListRemoteValue>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct HTMLCollectionRemoteValue {
        #[serde(rename = "type")]
        html_collection_remote_value_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<ListRemoteValue>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NodeRemoteValue {
        #[serde(rename = "type")]
        node_remote_value_type: String,
        #[serde(rename = "sharedId", skip_serializing_if = "Option::is_none")]
        shared_id: Option<SharedId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(rename = "internalId", skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Box<NodeProperties>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NodeProperties {
        #[serde(rename = "nodeType")]
        node_type: JsUint,
        #[serde(rename = "childNodeCount")]
        child_node_count: JsUint,
        #[serde(skip_serializing_if = "Option::is_none")]
        attributes: Option<HashMap<String, String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        children: Option<Vec<NodeRemoteValue>>,
        #[serde(rename = "localName", skip_serializing_if = "Option::is_none")]
        local_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode: Option<NodePropertiesMode>,
        #[serde(rename = "namespaceURI", skip_serializing_if = "Option::is_none")]
        namespace_uri: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "nodeValue")]
        node_value: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "shadowRoot")]
        shadow_root: Option<Option<NodeRemoteValue>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum NodePropertiesMode {
        Open,
        Closed,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct WindowProxyRemoteValue {
        #[serde(rename = "type")]
        window_proxy_remote_value_type: String,
        value: WindowProxyProperties,
        #[serde(skip_serializing_if = "Option::is_none")]
        handle: Option<Handle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        internal_id: Option<InternalId>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct WindowProxyProperties {
        context: browsing_context::BrowsingContext,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum ResultOwnership {
        Root,
        None,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SerializationOptions {
        #[serde(rename = "maxDomDepth", skip_serializing_if = "Option::is_none")]
        max_dom_depth: Option<JsUint>,
        #[serde(rename = "maxObjectDepth", skip_serializing_if = "Option::is_none")]
        max_object_depth: Option<JsUint>,
        #[serde(rename = "includeShadowTree", skip_serializing_if = "Option::is_none")]
        include_shadow_tree: Option<IncludeShadowTree>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum IncludeShadowTree {
        None,
        Open,
        All,
    }

    pub type SharedId = String;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StackFrame {
        #[serde(rename = "columnNumber")]
        column_number: JsUint,
        #[serde(rename = "functionName")]
        function_name: String,
        #[serde(rename = "lineNumber")]
        line_number: JsUint,
        url: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StackTrace {
        #[serde(rename = "callFrames")]
        call_frames: Vec<StackFrame>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RealmTarget {
        realm: Realm,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContextTarget {
        context: browsing_context::BrowsingContext,
        #[serde(skip_serializing_if = "Option::is_none")]
        sandbox: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Target {
        ContextTarget(ContextTarget),
        RealmTarget(RealmTarget),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AddPreloadScript {
        method: String,
        params: AddPreloadScriptParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AddPreloadScriptParameters {
        #[serde(rename = "functionDeclaration")]
        function_declaration: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<ChannelValue>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        contexts: Option<Vec<browsing_context::BrowsingContext>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        sandbox: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Disown {
        method: String,
        params: DisownParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DisownParameters {
        handles: Vec<Handle>,
        target: Target,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CallFunction {
        method: String,
        params: CallFunctionParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CallFunctionParameters {
        #[serde(rename = "functionDeclaration")]
        function_declaration: String,
        #[serde(rename = "awaitPromise")]
        await_promise: bool,
        target: Target,
        #[serde(skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<LocalValue>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "resultOwnership")]
        result_ownership: Option<ResultOwnership>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "serializationOptions")]
        serialization_options: Option<SerializationOptions>,
        #[serde(skip_serializing_if = "Option::is_none")]
        this: Option<LocalValue>,
        #[serde(rename = "userActivation", skip_serializing_if = "Option::is_none")]
        user_activation: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Evaluate {
        method: String,
        params: EvaluateParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct EvaluateParameters {
        expression: String,
        target: Target,
        #[serde(rename = "awaitPromise")]
        await_promise: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "resultOwnership")]
        result_ownership: Option<ResultOwnership>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "serializationOptions")]
        serialization_options: Option<SerializationOptions>,
        #[serde(rename = "userActivation", skip_serializing_if = "Option::is_none")]
        user_activation: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetRealms {
        method: String,
        params: GetRealmsParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetRealmsParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        context: Option<browsing_context::BrowsingContext>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "type")]
        realm_type: Option<RealmType>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RemovePreloadScript {
        method: String,
        params: RemovePreloadScriptParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RemovePreloadScriptParameters {
        script: PreloadScript,
    }
}

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub enum StorageCommand {
    DeleteCookies(storage::DeleteCookies),
    GetCookies(storage::GetCookies),
    SetCookie(storage::SetCookie),
}

pub mod storage {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PartionKey {
        #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
        user_context: Option<String>,
        #[serde(rename = "sourceOrigin", skip_serializing_if = "Option::is_none")]
        source_origin: Option<String>,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetCookies {
        method: String,
        params: GetCookiesParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CookieFilter {
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<network::BytesValue>,
        #[serde(skip_serializing_if = "Option::is_none")]
        domain: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<JsUint>,
        #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
        http_only: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        secure: Option<bool>,
        #[serde(rename = "sameSite", skip_serializing_if = "Option::is_none")]
        same_site: Option<network::SameSite>,
        #[serde(skip_serializing_if = "Option::is_none")]
        expiry: Option<JsUint>,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BrowsingContextPartitionDescriptor {
        #[serde(rename = "type")]
        browsing_context_partition_descriptor_type: String,
        context: browsing_context::BrowsingContext,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageKeyPartitionDescriptor {
        #[serde(rename = "type")]
        storage_key_partition_descriptor_type: String,
        #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
        user_context: Option<String>,
        #[serde(rename = "sourceOrigin", skip_serializing_if = "Option::is_none")]
        source_origin: Option<String>,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum PartitionDescriptor {
        BrowsingContextPartitionDescriptor(BrowsingContextPartitionDescriptor),
        StorageKeyPartitionDescriptor(StorageKeyPartitionDescriptor),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GetCookiesParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        filter: Option<CookieFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        partition: Option<PartitionDescriptor>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetCookie {
        method: String,
        params: SetCookieParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PartialCookie {
        name: String,
        value: network::BytesValue,
        domain: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
        #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
        http_only: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        secure: Option<bool>,
        #[serde(rename = "sameSite", skip_serializing_if = "Option::is_none")]
        same_site: Option<network::SameSite>,
        #[serde(skip_serializing_if = "Option::is_none")]
        expiry: Option<JsUint>,
        extensible: Extensible,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetCookieParameters {
        cookie: PartialCookie,
        #[serde(skip_serializing_if = "Option::is_none")]
        partition: Option<PartitionDescriptor>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeleteCookies {
        method: String,
        params: DeleteCookiesParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeleteCookiesParameters {
        #[serde(skip_serializing_if = "Option::is_none")]
        filter: Option<CookieFilter>,
        #[serde(skip_serializing_if = "Option::is_none")]
        partition: Option<PartitionDescriptor>,
    }
}

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub enum InputCommand {
    PerformActions(input::PerformActions),
    ReleaseActions(input::ReleaseActions),
    SetFiles(input::SetFiles),
}

pub mod input {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElementOrigin {
        #[serde(rename = "type")]
        element_origin_type: String,
        element: script::SharedReference,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PerformActions {
        method: String,
        params: PerformActionsParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PerformActionsParameters {
        context: browsing_context::BrowsingContext,
        actions: Vec<SourceActions>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum SourceActions {
        NoneSourceActions(NoneSourceActions),
        KeySourceActions(KeySourceActions),
        PointerSourceActions(PointerSourceActions),
        WheelSourceActions(WheelSourceActions),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NoneSourceActions {
        #[serde(rename = "type")]
        none_source_actions_type: String,
        id: String,
        actions: Vec<NoneSourceAction>,
    }

    pub type NoneSourceAction = PauseAction;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeySourceActions {
        #[serde(rename = "type")]
        key_source_actions_type: String,
        id: String,
        actions: Vec<KeySourceAction>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum KeySourceAction {
        PauseAction(PauseAction),
        KeyDownAction(KeyDownAction),
        KeyUpAction(KeyUpAction),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PointerSourceActions {
        #[serde(rename = "type")]
        pointer_source_actions_type: String,
        id: String,
        actions: Vec<PointerSourceAction>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum PointerType {
        Mouse,
        Pen,
        Touch,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PointerParameters {
        #[serde(rename = "pointerType", skip_serializing_if = "Option::is_none")]
        pointer_type: Option<PointerType>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum PointerSourceAction {
        PauseAction(PauseAction),
        PointerDownAction(PointerDownAction),
        PointerUpAction(PointerUpAction),
        PointerMoveAction(PointerMoveAction),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct WheelSourceActions {
        #[serde(rename = "type")]
        wheel_source_actions_type: String,
        id: String,
        actions: Vec<WheelSourceAction>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum WheelSourceAction {
        PauseAction(PauseAction),
        WheelScrollAction(WheelScrollAction),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PauseAction {
        #[serde(rename = "type")]
        pause_action_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<JsUint>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeyDownAction {
        #[serde(rename = "type")]
        key_down_action_type: String,
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeyUpAction {
        #[serde(rename = "type")]
        key_up_action_type: String,
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PointerUpAction {
        #[serde(rename = "type")]
        pointer_up_action_type: String,
        button: JsUint,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PointerDownAction {
        #[serde(rename = "type")]
        pointer_down_action_type: String,
        button: JsUint,
        #[serde(flatten)]
        pointer_common_properties: PointerCommonProperties,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PointerMoveAction {
        #[serde(rename = "type")]
        pointer_move_action_type: String,
        x: JsInt,
        y: JsInt,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<JsUint>,
        #[serde(skip_serializing_if = "Option::is_none")]
        origin: Option<Origin>,
        #[serde(flatten)]
        pointer_common_properties: PointerCommonProperties,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct WheelScrollAction {
        #[serde(rename = "type")]
        wheel_scroll_action_type: String,
        x: JsInt,
        y: JsInt,
        #[serde(rename = "deltaX")]
        delta_x: JsInt,
        #[serde(rename = "deltaY")]
        delta_y: JsInt,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<JsUint>,
        #[serde(skip_serializing_if = "Option::is_none")]
        origin: Option<Origin>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PointerCommonProperties {
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<JsUint>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<JsUint>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pressure: Option<f64>,
        #[serde(rename = "tangentialPressure", skip_serializing_if = "Option::is_none")]
        tangential_pressure: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        twist: Option<JsUint>,
        #[serde(rename = "altitudeAngle", skip_serializing_if = "Option::is_none")]
        altitude_angle: Option<f64>,
        #[serde(rename = "azimuthAngle", skip_serializing_if = "Option::is_none")]
        azimuth_angle: Option<f64>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "viewport")]
        Viewport,
        #[serde(rename = "pointer")]
        Pointer,
        ElementOrigin(ElementOrigin),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReleaseActions {
        method: String,
        params: ReleaseActionsParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReleaseActionsParameters {
        context: browsing_context::BrowsingContext,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetFiles {
        method: String,
        params: SetFilesParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SetFilesParameters {
        context: browsing_context::BrowsingContext,
        element: script::SharedReference,
        files: Vec<String>,
    }
}

// --------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub enum WebExtensionCommand {
    Install(web_extension::Install),
    Uninstall(web_extension::Uninstall),
}

pub mod web_extension {
    use super::*;

    pub type Extension = String;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Install {
        method: String,
        params: InstallParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InstallParameters {
        #[serde(rename = "extensionData")]
        extension_data: ExtensionData,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ExtensionData {
        ExtensionArchivePath(ExtensionArchivePath),
        ExtensionBase64Encoded(ExtensionBase64Encoded),
        ExtensionPath(ExtensionPath),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExtensionPath {
        #[serde(rename = "type")]
        extension_path_type: String,
        path: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExtensionArchivePath {
        #[serde(rename = "type")]
        extension_archive_path_type: String,
        path: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExtensionBase64Encoded {
        #[serde(rename = "type")]
        extension_base64_encoded_type: String,
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Uninstall {
        method: String,
        params: UninstallParameters,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct UninstallParameters {
        extension: Extension,
    }
}
