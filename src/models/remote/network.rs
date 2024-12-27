use crate::models::remote::browsing_context::BrowsingContext;
use crate::models::remote::{Extensible, JsInt, JsUint};
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
    contexts: Option<Vec<BrowsingContext>>,
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
    contexts: Option<Vec<BrowsingContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheBehavior {
    Default,
    Bypass,
}
