use crate::models::local::Extensible;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SessionResult {
    NewResult(NewResult),
    StatusResult(StatusResult),
}

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
