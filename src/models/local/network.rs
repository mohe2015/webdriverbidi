use crate::models::local::{browsing_context, script, Extensible, JsUint};
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
