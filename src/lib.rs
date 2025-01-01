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
    mod id;
}
pub mod command_sender;
pub mod error;
pub mod message_handler;

// Re-export key functionality
pub use models::local;
pub use models::remote;
