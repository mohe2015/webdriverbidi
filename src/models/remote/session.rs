use crate::remote::browsing_context::BrowsingContext;
use crate::remote::{EmptyParams, Extensible};
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
    pub always_match: Option<CapabilityRequest>,
    #[serde(rename = "firstMatch", skip_serializing_if = "Option::is_none")]
    pub first_match: Option<Vec<CapabilityRequest>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilityRequest {
    #[serde(
        rename = "acceptInsecureCerts",
        skip_serializing_if = "Option::is_none"
    )]
    pub accept_insecure_certs: Option<bool>,
    #[serde(rename = "browserName", skip_serializing_if = "Option::is_none")]
    pub browser_name: Option<String>,
    #[serde(rename = "browserVersion", skip_serializing_if = "Option::is_none")]
    pub browser_version: Option<String>,
    #[serde(rename = "platformName", skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<ProxyConfiguration>,
    #[serde(
        rename = "unhandledPromptBehavior",
        skip_serializing_if = "Option::is_none"
    )]
    pub unhandled_prompt_behavior: Option<UserPromptHandler>,
    pub extensible: Extensible,
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
    pub proxy_type: String,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManualProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(rename = "ftpProxy", skip_serializing_if = "Option::is_none")]
    pub ftp_proxy: Option<String>,
    #[serde(rename = "httpProxy", skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<String>,
    #[serde(rename = "sslProxy", skip_serializing_if = "Option::is_none")]
    pub ssl_proxy: Option<String>,
    #[serde(
        rename = "socksProxyConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub socks_proxy_configuration: Option<SocksProxyConfiguration>,
    #[serde(rename = "noProxy", skip_serializing_if = "Option::is_none")]
    pub no_proxy: Option<Vec<String>>,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocksProxyConfiguration {
    #[serde(rename = "socksProxy")]
    pub socks_proxy: String,
    #[serde(rename = "socksVersion")]
    pub socks_version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(rename = "proxyAutoconfigUrl")]
    pub proxy_autoconfig_url: String,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPromptHandler {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<UserPromptHandlerType>,
    #[serde(rename = "beforeUnload", skip_serializing_if = "Option::is_none")]
    pub before_unload: Option<UserPromptHandlerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<UserPromptHandlerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<UserPromptHandlerType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<UserPromptHandlerType>,
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
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeByIDRequest {
    pub subscriptions: Vec<Subscription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeByAttributesRequest {
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub method: String,
    pub params: EmptyParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct New {
    pub method: String,
    pub params: NewParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewParameters {
    pub capabilities: CapabilitiesRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct End {
    pub method: String,
    pub params: EmptyParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub method: String,
    pub params: SubscriptionRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unsubscribe {
    pub method: String,
    pub params: UnsubscribeRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnsubscribeRequest {
    UnsubscribeByAttributesRequest(UnsubscribeByAttributesRequest),
    UnsubscribeByIDRequest(UnsubscribeByIDRequest),
}
