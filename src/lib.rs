pub mod capabilities;
pub mod http_session;
pub mod session;
pub mod commands_tmp;
mod commands {
    pub mod local;
    pub mod remote;
}

// Re-export key functionality
pub use capabilities::{Capabilities, CapabilitiesBuilder, Capability};
pub use http_session::{start_session, SessionResponse};
