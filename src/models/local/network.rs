use crate::local::{browsing_context, script, Extensible, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkResult {
    AddInterceptResult(AddInterceptResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkEvent {
    AuthRequired(AuthRequired),
    BeforeRequestSent(BeforeRequestSent),
    FetchError(FetchError),
    ResponseCompleted(ResponseCompleted),
    ResponseStarted(ResponseStarted),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthChallenge {
    pub scheme: String,
    pub realm: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseParameters {
    pub context: Option<browsing_context::BrowsingContext>,
    pub is_blocked: bool,
    pub navigation: Option<browsing_context::Navigation>,
    pub redirect_count: JsUint,
    pub request: RequestData,
    pub timestamp: JsUint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intercepts: Option<Vec<Intercept>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BytesValue {
    StringValue(StringValue),
    Base64Value(Base64Value),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StringValue {
    #[serde(rename = "type")]
    pub value_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Base64Value {
    #[serde(rename = "type")]
    pub value_type: String,
    pub value: String,
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
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchTimingInfo {
    #[serde(rename = "timeOrigin")]
    pub time_origin: f64,
    #[serde(rename = "requestTime")]
    pub request_time: f64,
    #[serde(rename = "redirectStart")]
    pub redirect_start: f64,
    #[serde(rename = "redirectEnd")]
    pub redirect_end: f64,
    #[serde(rename = "fetchStart")]
    pub fetch_start: f64,
    #[serde(rename = "dnsStart")]
    pub dns_start: f64,
    #[serde(rename = "dnsEnd")]
    pub dns_end: f64,
    #[serde(rename = "connectStart")]
    pub connect_start: f64,
    #[serde(rename = "connectEnd")]
    pub connect_end: f64,
    #[serde(rename = "tlsStart")]
    pub tls_start: f64,
    #[serde(rename = "requestStart")]
    pub request_start: f64,
    #[serde(rename = "responseStart")]
    pub response_start: f64,
    #[serde(rename = "responseEnd")]
    pub response_end: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub name: String,
    pub value: BytesValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Initiator {
    #[serde(rename = "columnNumber", skip_serializing_if = "Option::is_none")]
    pub column_number: Option<JsUint>,
    #[serde(rename = "lineNumber", skip_serializing_if = "Option::is_none")]
    pub line_number: Option<JsUint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    #[serde(rename = "stackTrace", skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<script::StackTrace>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<InitiatorType>,
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
    pub request: Request,
    pub url: String,
    pub method: String,
    pub headers: Vec<Header>,
    pub cookies: Vec<Cookie>,
    #[serde(rename = "headersSize")]
    pub headers_size: JsUint,
    #[serde(rename = "bodySize")]
    pub body_size: Option<JsUint>,
    pub destination: String,
    #[serde(rename = "initiatorType")]
    pub initiator_type: Option<String>,
    pub timings: FetchTimingInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseContent {
    pub size: JsUint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    pub url: String,
    pub protocol: String,
    pub status: JsUint,
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "fromCache")]
    pub from_cache: bool,
    pub headers: Vec<Header>,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "bytesReceived")]
    pub bytes_received: JsUint,
    #[serde(rename = "headersSize")]
    pub headers_size: Option<JsUint>,
    #[serde(rename = "bodySize")]
    pub body_size: Option<JsUint>,
    pub content: ResponseContent,
    #[serde(rename = "authChallenges", skip_serializing_if = "Option::is_none")]
    pub auth_challenges: Option<Vec<AuthChallenge>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddInterceptResult {
    pub intercept: Intercept,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthRequired {
    pub method: String,
    pub params: AuthRequiredParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequiredParameters {
    #[serde(flatten)]
    pub base: BaseParameters,
    pub response: ResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeforeRequestSent {
    pub method: String,
    pub params: BeforeRequestSentParameters,
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
    pub method: String,
    pub params: FetchErrorParameters,
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
    pub method: String,
    pub params: ResponseCompletedParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCompletedParameters {
    #[serde(flatten)]
    pub base: BaseParameters,
    pub response: ResponseData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStarted {
    pub method: String,
    pub params: ResponseStartedParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseStartedParameters {
    #[serde(flatten)]
    pub base: BaseParameters,
    pub response: ResponseData,
}
