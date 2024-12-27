use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;

pub type Extensible = HashMap<String, Value>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    browser_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_insecure_certs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_load_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set_window_rect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_file_interactability: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhandled_prompt_behavior: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<String>,
    // Ensure the webSocketUrl capability is always present and set to true
    #[serde(default = "default_web_socket_url")]
    web_socket_url: bool,
    #[serde(flatten)]
    extension: Extensible,
}

fn default_web_socket_url() -> bool {
    true
}

impl CapabilityRequest {
    pub fn new() -> Self {
        Self {
            browser_name: None,
            browser_version: None,
            platform_name: None,
            accept_insecure_certs: None,
            page_load_strategy: None,
            proxy: None,
            set_window_rect: None,
            timeouts: None,
            strict_file_interactability: None,
            unhandled_prompt_behavior: None,
            user_agent: None,
            web_socket_url: true,
            extension: Extensible::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    #[serde(rename = "alwaysMatch", skip_serializing_if = "Option::is_none")]
    always_match: Option<CapabilityRequest>,
    #[serde(rename = "firstMatch", skip_serializing_if = "Option::is_none")]
    first_match: Option<Vec<HashMap<String, Value>>>,
}

impl Capabilities {
    pub fn new(mut always_match: CapabilityRequest) -> Self {
        always_match.web_socket_url = true;
        Self {
            always_match: Some(always_match),
            first_match: None,
        }
    }
    
    pub fn add_extension(&mut self, key: String, value: Value) {
        if key != "webSocketUrl" {
            if let Some(ref mut always_match) = self.always_match {
                always_match.extension.insert(key, value);
            }
        }
    }

    pub fn add_first_match(&mut self, first_match: HashMap<String, Value>) {
        if let Some(ref mut first_match_vec) = self.first_match {
            first_match_vec.push(first_match);
        } else {
            self.first_match = Some(vec![first_match]);
        }
    }
    
    pub fn build(&self) -> Value {
        let json = json!({
            "capabilities": self
        });
        json
    }
}








// /// Enum for standard capabilities
// /// https://w3c.github.io/webdriver/#dfn-table-of-standard-capabilities
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub enum Capability {
//     BrowserName(String),
//     BrowserVersion(String),
//     PlatformName(String),
//     AcceptInsecureCerts(bool),
//     PageLoadStrategy(String),
//     Proxy(Value),
//     SetWindowRect(bool),
//     Timeouts(Value),
//     StrictFileInteractability(bool),
//     UnhandledPromptBehavior(String),
//     UserAgent(String),
//     // Defined by WebDriver BiDi https://w3c.github.io/webdriver-bidi/#type-session-CapabilityRequest
//     WebSocketUrl(bool),
// }

// /// Struct for session capabilities
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Capabilities {
//     pub always_match: HashMap<String, Value>,
//     pub first_match: Vec<HashMap<String, Value>>,
// }

// /// Builder for constructing capabilities
// #[derive(Debug, Default)]
// pub struct CapabilitiesBuilder {
//     always_match: HashMap<String, Value>,
//     first_match: Vec<HashMap<String, Value>>,
// }

// impl CapabilitiesBuilder {
//     /// Creates a new builder
//     pub fn new() -> Self {
//         Self::default()
//     }

//     /// Adds a standard capability
//     pub fn add_standard(&mut self, capability: Capability) -> &mut Self {
//         let (key, value) = match capability {
//             Capability::BrowserName(name) => ("browserName", Value::String(name)),
//             Capability::BrowserVersion(version) => ("browserVersion", Value::String(version)),
//             Capability::PlatformName(platform) => ("platformName", Value::String(platform)),
//             Capability::AcceptInsecureCerts(accept) => ("acceptInsecureCerts", Value::Bool(accept)),
//             Capability::PageLoadStrategy(strategy) => ("pageLoadStrategy", Value::String(strategy)),
//             Capability::Proxy(proxy) => ("proxy", proxy),
//             Capability::SetWindowRect(set) => ("setWindowRect", Value::Bool(set)),
//             Capability::Timeouts(timeouts) => ("timeouts", timeouts),
//             Capability::StrictFileInteractability(strict) => {
//                 ("strictFileInteractability", Value::Bool(strict))
//             }
//             Capability::UnhandledPromptBehavior(behavior) => {
//                 ("unhandledPromptBehavior", Value::String(behavior))
//             }
//             Capability::UserAgent(user_agent) => ("userAgent", Value::String(user_agent)),
//             Capability::WebSocketUrl(websocket_url) => ("webSocketUrl", Value::Bool(websocket_url)),
//         };
//         self.always_match.insert(key.to_string(), value);
//         self
//     }

//     /// Adds a vendor-specific capability
//     pub fn add_vendor(&mut self, key: &str, value: Value) -> &mut Self {
//         self.always_match.insert(key.to_string(), value);
//         self
//     }

//     /// Adds a firstMatch capability set
//     pub fn add_first_match(&mut self, match_set: HashMap<String, Value>) -> &mut Self {
//         self.first_match.push(match_set);
//         self
//     }

//     /// Builds the `Capabilities` object
//     pub fn build(&self) -> Capabilities {
//         Capabilities {
//             always_match: self.always_match.clone(),
//             first_match: self.first_match.clone(),
//         }
//     }
// }

// // TODO: Add default capabilities if none are provided
// // TODO: Ensure the webSocketUrl capability is always present and set to true
// // TODO: Validate user-provided capabilities against WebDriver specifications
