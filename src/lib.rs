pub mod webdriver {
    pub mod capabilities;
    pub mod session;
}
pub mod session;
pub mod models {
    pub mod local;
    pub mod remote;
}
pub mod commands {
    pub mod browsing_context;
}

// Re-export key functionality
pub use webdriver::capabilities::{CapabilityRequest, Capabilities};
