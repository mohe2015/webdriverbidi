use crate::remote::browsing_context::BrowsingContext;
use crate::remote::{Extensible, JsInt, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkCommand {
    AddIntercept(AddIntercept),
    ContinueRequest(ContinueRequest),
    ContinueResponse(ContinueResponse),
    ContinueWithAuth(ContinueWithAuth),
    FailRequest(FailRequest),
    ProvideResponse(ProvideResponse),
    RemoveIntercept(RemoveIntercept),
    SetCacheBehavior(SetCacheBehavior),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthCredentials {
    #[serde(rename = "type")]
    pub auth_credentials_type: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BytesValue {
    StringValue(StringValue),
    Base64Value(Base64Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
    #[serde(rename = "type")]
    pub string_value_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64Value {
    #[serde(rename = "type")]
    pub base64_value_type: String,
    pub value: String,
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
    pub name: String,
    pub value: BytesValue,
    pub domain: String,
    pub path: String,
    pub size: JsUint,
    #[serde(rename = "httpOnly")]
    pub http_only: bool,
    pub secure: bool,
    #[serde(rename = "sameSite")]
    pub same_site: SameSite,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<JsUint>,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookieHeader {
    pub name: String,
    pub value: BytesValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: BytesValue,
}

pub type Intercept = String;
pub type Request = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookieHeader {
    pub name: String,
    pub value: BytesValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(rename = "maxAge", skip_serializing_if = "Option::is_none")]
    pub max_age: Option<JsInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "sameSite", skip_serializing_if = "Option::is_none")]
    pub same_site: Option<SameSite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UrlPattern {
    UrlPatternPattern(UrlPatternPattern),
    UrlPatternString(UrlPatternString),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlPatternPattern {
    #[serde(rename = "type")]
    pub url_pattern_pattern_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pathname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlPatternString {
    #[serde(rename = "type")]
    pub url_pattern_string_type: String,
    pub pattern: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddIntercept {
    pub method: String,
    pub params: AddInterceptParameters,
}

impl AddIntercept {
    pub fn new(params: AddInterceptParameters) -> Self {
        Self {
            method: "network.addIntercept".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddInterceptParameters {
    pub phases: Vec<InterceptPhase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_patterns: Option<Vec<UrlPattern>>,
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
    pub method: String,
    pub params: ContinueRequestParameters,
}

impl ContinueRequest {
    pub fn new(params: ContinueRequestParameters) -> Self {
        Self {
            method: "network.continueRequest".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueRequestParameters {
    pub request: Request,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<BytesValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<CookieHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueResponse {
    pub method: String,
    pub params: ContinueResponseParameters,
}

impl ContinueResponse {
    pub fn new(params: ContinueResponseParameters) -> Self {
        Self {
            method: "network.continueResponse".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueResponseParameters {
    pub request: Request,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<SetCookieHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<AuthCredentials>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,
    #[serde(rename = "reasonPhrase", skip_serializing_if = "Option::is_none")]
    pub reason_phrase: Option<String>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<JsUint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuth {
    pub method: String,
    pub params: ContinueWithAuthParameters,
}

impl ContinueWithAuth {
    pub fn new(params: ContinueWithAuthParameters) -> Self {
        Self {
            method: "network.continueWithAuth".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthParameters {
    pub request: Request,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub auth_option: Option<ContinueWithAuthOption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContinueWithAuthOption {
    Credentials(ContinueWithAuthCredentials),
    NoCredentials(ContinueWithAuthNoCredentials),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthCredentials {
    pub action: String,
    pub credentials: AuthCredentials,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthNoCredentials {
    pub action: NoCredentialsAction,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NoCredentialsAction {
    Default,
    Cancel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailRequest {
    pub method: String,
    pub params: FailRequestParameters,
}

impl FailRequest {
    pub fn new(params: FailRequestParameters) -> Self {
        Self {
            method: "network.failRequest".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailRequestParameters {
    pub request: Request,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvideResponse {
    pub method: String,
    pub params: ProvideResponseParameters,
}

impl ProvideResponse {
    pub fn new(params: ProvideResponseParameters) -> Self {
        Self {
            method: "network.provideResponse".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvideResponseParameters {
    pub request: Request,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<BytesValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<SetCookieHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,
    #[serde(rename = "reasonPhrase", skip_serializing_if = "Option::is_none")]
    pub reason_phrase: Option<String>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<JsUint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveIntercept {
    pub method: String,
    pub params: RemoveInterceptParameters,
}

impl RemoveIntercept {
    pub fn new(params: RemoveInterceptParameters) -> Self {
        Self {
            method: "network.removeIntercept".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveInterceptParameters {
    pub intercept: Intercept,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCacheBehavior {
    pub method: String,
    pub params: SetCacheBehaviorParameters,
}

impl SetCacheBehavior {
    pub fn new(params: SetCacheBehaviorParameters) -> Self {
        Self {
            method: "network.setCacheBehavior".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCacheBehaviorParameters {
    pub cache_behavior: CacheBehavior,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheBehavior {
    Default,
    Bypass,
}
