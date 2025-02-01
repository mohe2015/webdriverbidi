#![allow(clippy::all)]

use crate::remote::browsing_context::BrowsingContext;
use crate::remote::{Extensible, JsInt, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl AuthCredentials {
    pub fn new(username: String, password: String) -> Self {
        Self {
            auth_credentials_type: "password".to_string(),
            username,
            password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl StringValue {
    pub fn new(value: String) -> Self {
        Self {
            string_value_type: "string".to_string(),
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64Value {
    #[serde(rename = "type")]
    pub base64_value_type: String,
    pub value: String,
}

impl Base64Value {
    pub fn new(value: String) -> Self {
        Self {
            base64_value_type: "base64".to_string(),
            value,
        }
    }
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

impl Cookie {
    pub fn new(
        name: String,
        value: BytesValue,
        domain: String,
        path: String,
        size: JsUint,
        http_only: bool,
        secure: bool,
        same_site: SameSite,
        expiry: Option<JsUint>,
        extensible: Extensible,
    ) -> Self {
        Self {
            name,
            value,
            domain,
            path,
            size,
            http_only,
            secure,
            same_site,
            expiry,
            extensible,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookieHeader {
    pub name: String,
    pub value: BytesValue,
}

impl CookieHeader {
    pub fn new(name: String, value: BytesValue) -> Self {
        Self { name, value }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: BytesValue,
}

impl Header {
    pub fn new(name: String, value: BytesValue) -> Self {
        Self { name, value }
    }
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

impl SetCookieHeader {
    pub fn new(
        name: String,
        value: BytesValue,
        domain: Option<String>,
        http_only: Option<bool>,
        expiry: Option<String>,
        max_age: Option<JsInt>,
        path: Option<String>,
        same_site: Option<SameSite>,
        secure: Option<bool>,
    ) -> Self {
        Self {
            name,
            value,
            domain,
            http_only,
            expiry,
            max_age,
            path,
            same_site,
            secure,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl UrlPatternPattern {
    pub fn new(
        protocol: Option<String>,
        hostname: Option<String>,
        port: Option<String>,
        pathname: Option<String>,
        search: Option<String>,
    ) -> Self {
        Self {
            url_pattern_pattern_type: "pattern".to_string(),
            protocol,
            hostname,
            port,
            pathname,
            search,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlPatternString {
    #[serde(rename = "type")]
    pub url_pattern_string_type: String,
    pub pattern: String,
}

impl UrlPatternString {
    pub fn new(pattern: String) -> Self {
        Self {
            url_pattern_string_type: "string".to_string(),
            pattern,
        }
    }
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

impl AddInterceptParameters {
    pub fn new(
        phases: Vec<InterceptPhase>,
        contexts: Option<Vec<BrowsingContext>>,
        url_patterns: Option<Vec<UrlPattern>>,
    ) -> Self {
        Self {
            phases,
            contexts,
            url_patterns,
        }
    }
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

impl ContinueRequestParameters {
    pub fn new(
        request: Request,
        body: Option<BytesValue>,
        cookies: Option<Vec<CookieHeader>>,
        headers: Option<Vec<Header>>,
        method: Option<String>,
        url: Option<String>,
    ) -> Self {
        Self {
            request,
            body,
            cookies,
            headers,
            method,
            url,
        }
    }
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

impl ContinueResponseParameters {
    pub fn new(
        request: Request,
        cookies: Option<Vec<SetCookieHeader>>,
        credentials: Option<AuthCredentials>,
        headers: Option<Vec<Header>>,
        reason_phrase: Option<String>,
        status_code: Option<JsUint>,
    ) -> Self {
        Self {
            request,
            cookies,
            credentials,
            headers,
            reason_phrase,
            status_code,
        }
    }
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

impl ContinueWithAuthParameters {
    pub fn new(request: Request, auth_option: Option<ContinueWithAuthOption>) -> Self {
        Self {
            request,
            auth_option,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContinueWithAuthOption {
    Credentials(ContinueWithAuthCredentials),
    NoCredentials(ContinueWithAuthNoCredentials),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthCredentials {
    pub action: String,
    pub credentials: AuthCredentials,
}

impl ContinueWithAuthCredentials {
    pub fn new(action: String, credentials: AuthCredentials) -> Self {
        Self {
            action,
            credentials,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContinueWithAuthNoCredentials {
    pub action: NoCredentialsAction,
}

impl ContinueWithAuthNoCredentials {
    pub fn new(action: NoCredentialsAction) -> Self {
        Self { action }
    }
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

impl FailRequestParameters {
    pub fn new(request: Request) -> Self {
        Self { request }
    }
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

impl ProvideResponseParameters {
    pub fn new(
        request: Request,
        body: Option<BytesValue>,
        cookies: Option<Vec<SetCookieHeader>>,
        headers: Option<Vec<Header>>,
        reason_phrase: Option<String>,
        status_code: Option<JsUint>,
    ) -> Self {
        Self {
            request,
            body,
            cookies,
            headers,
            reason_phrase,
            status_code,
        }
    }
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

impl RemoveInterceptParameters {
    pub fn new(intercept: Intercept) -> Self {
        Self { intercept }
    }
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

impl SetCacheBehaviorParameters {
    pub fn new(cache_behavior: CacheBehavior, contexts: Option<Vec<BrowsingContext>>) -> Self {
        Self {
            cache_behavior,
            contexts,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheBehavior {
    Default,
    Bypass,
}
