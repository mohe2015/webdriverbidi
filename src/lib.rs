pub mod capabilities;
pub mod commands_tmp;
pub mod http_session;
pub mod session;
pub mod models {
    pub mod local;
    pub mod remote;
}
pub mod commands {
    pub mod browsing_context;
}

// Re-export key functionality
pub use capabilities::{Capabilities, CapabilitiesBuilder, Capability};
pub use http_session::{start_session, SessionResponse};
