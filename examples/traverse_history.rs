use tokio;
use tokio::time;

// --------------------------------------------------

use webdriverbidi::remote::browsing_context::{
    GetTreeParameters, NavigateParameters, ReadinessState, TraverseHistoryParameters,
};
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::webdriver::capabilities::{Capabilities, CapabilityRequest};

// --------------------------------------------------

async fn navigate(session: &mut WebDriverBiDiSession, context: String, url: &str) {
    let navigate_params = NavigateParameters::new(
        context.clone(),
        url.to_owned(),
        Some(ReadinessState::Complete),
    );
    session
        .browsing_context_navigate(navigate_params)
        .await
        .expect("Failed to send command");
}

async fn sleep(secs: u64) {
    time::sleep(time::Duration::from_secs(secs)).await
}

async fn traverse_history(session: &mut WebDriverBiDiSession, context: String, delta: i64) {
    let traverse_history_params = TraverseHistoryParameters::new(context.clone(), delta);
    session
        .browsing_context_traverse_history(traverse_history_params)
        .await
        .expect("Failed to send command");
}

#[tokio::main]
async fn main() {
    // Define the capabilities for the WebDriver session
    let always_match = CapabilityRequest::new();
    let capabilities = Capabilities::new(always_match);

    // Initialize a new WebDriver BiDi session and start it
    let host = String::from("localhost");
    let port = 4444;
    let mut bidi_session = WebDriverBiDiSession::new(host, port, capabilities);
    let _ = bidi_session.start().await.expect("Failed to start session");

    // Get the browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = bidi_session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to send command");

    // Browsing context ID
    let context = get_tree_rslt.contexts[0].context.clone();

    // Navigate to rust-lang.org
    navigate(
        &mut bidi_session,
        context.clone(),
        "https://www.rust-lang.org/",
    )
    .await;
    sleep(2).await;

    // Navigate to crates.io
    navigate(&mut bidi_session, context.clone(), "https://crates.io/").await;
    sleep(2).await;

    // Go back to rust-lang.org
    traverse_history(&mut bidi_session, context.clone(), -1).await;
    sleep(2).await;

    // Go forward to crates.io
    traverse_history(&mut bidi_session, context, 1).await;
    sleep(2).await;

    // Close the session
    bidi_session.close().await.expect("Failed to close session");
}
