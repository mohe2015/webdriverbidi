use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Enum for standard capabilities
/// https://w3c.github.io/webdriver/#dfn-table-of-standard-capabilities
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Capability {
    BrowserName(String),
    BrowserVersion(String),
    PlatformName(String),
    AcceptInsecureCerts(bool),
    PageLoadStrategy(String),
    Proxy(Value),
    SetWindowRect(bool),
    Timeouts(Value),
    StrictFileInteractability(bool),
    UnhandledPromptBehavior(String),
    UserAgent(String),
    // Defined by WebDriver BiDi https://w3c.github.io/webdriver-bidi/#type-session-CapabilityRequest
    WebSocketUrl(bool),
}

/// Struct for session capabilities
#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    pub always_match: HashMap<String, Value>,
    pub first_match: Vec<HashMap<String, Value>>,
}

/// Builder for constructing capabilities
#[derive(Debug, Default)]
pub struct CapabilitiesBuilder {
    always_match: HashMap<String, Value>,
    first_match: Vec<HashMap<String, Value>>,
}

impl CapabilitiesBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a standard capability
    pub fn add_standard(&mut self, capability: Capability) -> &mut Self {
        let (key, value) = match capability {
            Capability::BrowserName(name) => ("browserName", Value::String(name)),
            Capability::BrowserVersion(version) => ("browserVersion", Value::String(version)),
            Capability::PlatformName(platform) => ("platformName", Value::String(platform)),
            Capability::AcceptInsecureCerts(accept) => ("acceptInsecureCerts", Value::Bool(accept)),
            Capability::PageLoadStrategy(strategy) => ("pageLoadStrategy", Value::String(strategy)),
            Capability::Proxy(proxy) => ("proxy", proxy),
            Capability::SetWindowRect(set) => ("setWindowRect", Value::Bool(set)),
            Capability::Timeouts(timeouts) => ("timeouts", timeouts),
            Capability::StrictFileInteractability(strict) => {
                ("strictFileInteractability", Value::Bool(strict))
            }
            Capability::UnhandledPromptBehavior(behavior) => {
                ("unhandledPromptBehavior", Value::String(behavior))
            }
            Capability::UserAgent(user_agent) => ("userAgent", Value::String(user_agent)),
            Capability::WebSocketUrl(websocket_url) => ("webSocketUrl", Value::Bool(websocket_url)),
        };
        self.always_match.insert(key.to_string(), value);
        self
    }

    /// Adds a vendor-specific capability
    pub fn add_vendor(&mut self, key: &str, value: Value) -> &mut Self {
        self.always_match.insert(key.to_string(), value);
        self
    }

    /// Adds a firstMatch capability set
    pub fn add_first_match(&mut self, match_set: HashMap<String, Value>) -> &mut Self {
        self.first_match.push(match_set);
        self
    }

    /// Builds the `Capabilities` object
    pub fn build(&self) -> Capabilities {
        Capabilities {
            always_match: self.always_match.clone(),
            first_match: self.first_match.clone(),
        }
    }
}

// TODO: Add default capabilities if none are provided
// TODO: Ensure the webSocketUrl capability is always present and set to true
// TODO: Ensure user-provided capabilities match WebDriver specifications