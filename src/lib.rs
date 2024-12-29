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
    mod id;
    pub mod browsing_context;
    // pub mod error;
}
pub mod message_handler;
pub mod command_sender;
pub mod error;

// Re-export key functionality
pub use webdriver::capabilities::{CapabilityRequest, Capabilities};
