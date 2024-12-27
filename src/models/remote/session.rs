use crate::models::remote::browsing_context::BrowsingContext;
use crate::models::remote::{EmptyParams, Extensible};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionCommand {
    End(End),
    New(New),
    Status(Status),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
}

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
    contexts: Option<Vec<BrowsingContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeByIDRequest {
    subscriptions: Vec<Subscription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeByAttributesRequest {
    events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contexts: Option<Vec<BrowsingContext>>,
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
