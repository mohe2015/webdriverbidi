use tokio;
use webdriverbidi::remote::browsing_context::{
    GetTreeParameters, NavigateParameters, ReadinessState,
};
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::{Capabilities, CapabilityRequest};

#[tokio::main]
async fn main() {
    // Step 1: Define the capabilities for the WebDriver session
    let always_match = CapabilityRequest::new();
    let capabilities = Capabilities::new(always_match);

    // Step 2: Create a new WebDriver BiDi session and start it
    let mut bidi_session = WebDriverBiDiSession::new("localhost".to_string(), 4444, capabilities);
    let _ = bidi_session.start().await.expect("Failed to start session");

    // Step 3: Get the browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = bidi_session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to send command");

    // Step 4: Navigate to the Rust programming language website
    let navigate_params = NavigateParameters::new(
        get_tree_rslt.contexts[0].context.clone(),
        "https://www.rust-lang.org/".to_string(),
        Some(ReadinessState::Complete),
    );
    let _ = bidi_session
        .browsing_context_navigate(navigate_params)
        .await
        .expect("Failed to send command");

    // Admire the page for a few seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Step 5: Close the session
    bidi_session.close().await.expect("Failed to close session");
}
