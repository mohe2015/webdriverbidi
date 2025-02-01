#![allow(clippy::all)]

use crate::remote::browser::UserContext;
use crate::remote::browsing_context::BrowsingContext;
use crate::remote::{EmptyParams, Extensible};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl CapabilitiesRequest {
    pub fn new(
        always_match: Option<CapabilityRequest>,
        first_match: Option<Vec<CapabilityRequest>>,
    ) -> Self {
        Self {
            always_match,
            first_match,
        }
    }
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

impl CapabilityRequest {
    pub fn new(
        accept_insecure_certs: Option<bool>,
        browser_name: Option<String>,
        browser_version: Option<String>,
        platform_name: Option<String>,
        proxy: Option<ProxyConfiguration>,
        unhandled_prompt_behavior: Option<UserPromptHandler>,
        extensible: Extensible,
    ) -> Self {
        Self {
            accept_insecure_certs,
            browser_name,
            browser_version,
            platform_name,
            proxy,
            unhandled_prompt_behavior,
            extensible,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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

impl AutodetectProxyConfiguration {
    pub fn new(proxy_type: String, extensible: Extensible) -> Self {
        Self {
            proxy_type,
            extensible,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    pub extensible: Extensible,
}

impl DirectProxyConfiguration {
    pub fn new(proxy_type: String, extensible: Extensible) -> Self {
        Self {
            proxy_type,
            extensible,
        }
    }
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

impl ManualProxyConfiguration {
    pub fn new(
        proxy_type: String,
        ftp_proxy: Option<String>,
        http_proxy: Option<String>,
        ssl_proxy: Option<String>,
        socks_proxy_configuration: Option<SocksProxyConfiguration>,
        no_proxy: Option<Vec<String>>,
        extensible: Extensible,
    ) -> Self {
        Self {
            proxy_type,
            ftp_proxy,
            http_proxy,
            ssl_proxy,
            socks_proxy_configuration,
            no_proxy,
            extensible,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocksProxyConfiguration {
    #[serde(rename = "socksProxy")]
    pub socks_proxy: String,
    #[serde(rename = "socksVersion")]
    pub socks_version: u8,
}

impl SocksProxyConfiguration {
    pub fn new(socks_proxy: String, socks_version: u8) -> Self {
        Self {
            socks_proxy,
            socks_version,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(rename = "proxyAutoconfigUrl")]
    pub proxy_autoconfig_url: String,
    pub extensible: Extensible,
}

impl PacProxyConfiguration {
    pub fn new(proxy_type: String, proxy_autoconfig_url: String, extensible: Extensible) -> Self {
        Self {
            proxy_type,
            proxy_autoconfig_url,
            extensible,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    pub extensible: Extensible,
}

impl SystemProxyConfiguration {
    pub fn new(proxy_type: String, extensible: Extensible) -> Self {
        Self {
            proxy_type,
            extensible,
        }
    }
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

impl UserPromptHandler {
    pub fn new(
        alert: Option<UserPromptHandlerType>,
        before_unload: Option<UserPromptHandlerType>,
        confirm: Option<UserPromptHandlerType>,
        default: Option<UserPromptHandlerType>,
        prompt: Option<UserPromptHandlerType>,
    ) -> Self {
        Self {
            alert,
            before_unload,
            confirm,
            default,
            prompt,
        }
    }
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

impl SubscriptionRequest {
    pub fn new(
        events: Vec<String>,
        contexts: Option<Vec<BrowsingContext>>,
        user_contexts: Option<Vec<UserContext>>,
    ) -> Self {
        Self {
            events,
            contexts,
            user_contexts,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsubscribeByIDRequest {
    pub subscriptions: Vec<Subscription>,
}

impl UnsubscribeByIDRequest {
    pub fn new(subscriptions: Vec<Subscription>) -> Self {
        Self { subscriptions }
    }
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

impl Status {
    pub fn new(params: EmptyParams) -> Self {
        Self {
            method: "session.status".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct New {
    pub method: String,
    pub params: NewParameters,
}

impl New {
    pub fn new(params: NewParameters) -> Self {
        Self {
            method: "session.new".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewParameters {
    pub capabilities: CapabilitiesRequest,
}

impl NewParameters {
    pub fn new(capabilities: CapabilitiesRequest) -> Self {
        Self { capabilities }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct End {
    pub method: String,
    pub params: EmptyParams,
}

impl End {
    pub fn new(params: EmptyParams) -> Self {
        Self {
            method: "session.end".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub method: String,
    pub params: SubscriptionRequest,
}

impl Subscribe {
    pub fn new(params: SubscriptionRequest) -> Self {
        Self {
            method: "session.subscribe".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unsubscribe {
    pub method: String,
    pub params: UnsubscribeParameters,
}

impl Unsubscribe {
    pub fn new(params: UnsubscribeParameters) -> Self {
        Self {
            method: "session.unsubscribe".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnsubscribeParameters {
    UnsubscribeByAttributesRequest(UnsubscribeByAttributesRequest),
    UnsubscribeByIDRequest(UnsubscribeByIDRequest),
}
