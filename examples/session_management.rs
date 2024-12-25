use tokio;
use webdriverbidi::capabilities::{CapabilitiesBuilder, Capability};
use webdriverbidi::commands_tmp::get_tree_command;
use webdriverbidi::http_session::{close_session, start_session};
use webdriverbidi::session::WebDriverBiDiSession;

use webdriverbidi::commands::browsing_context::NavigateCommand;
use webdriverbidi::models::remote::browsing_context::{
    Navigate, NavigateParameters, ReadinessState,
};
#[tokio::main]
async fn main() {
    // Base URL of the WebDriver server (GeckoDriver, ChromeDriver or MSEdgeDriver)
    let base_url = "http://localhost:4444";

    let capabilities = CapabilitiesBuilder::new()
        .add_standard(Capability::WebSocketUrl(true))
        .build();

    // Step 1: Start a new session
    let session = start_session(base_url, capabilities)
        .await
        .expect("Failed to start WebDriver session");
    println!("Session started with ID: {}", session.session_id);
    println!("WebSocket URL: {}", session.websocket_url);

    // Step 2: Connect to the WebSocket
    let mut bidi_session = WebDriverBiDiSession::new(session.websocket_url.clone())
        .await
        .expect("Failed to connect to WebSocket");

    // Step 3: Send the `browsingContext.getTree` command
    let get_tree = get_tree_command(1);
    bidi_session
        .send_command(get_tree)
        .await
        .expect("Failed to send command");

    let response = bidi_session
        .receive_response()
        .await
        .expect("Failed to receive response");
    println!("Received getTree response: {}", response);

    // Step 4: Extract browsingContextId and navigate
    if let Some(context_id) = response["result"]["contexts"][0]["context"].as_str() {
        let navigate_params = NavigateParameters::new(
            context_id.to_string(),
            "https://www.rust-lang.org/".to_string(),
            Some(ReadinessState::Complete),
        );
        let navigate = Navigate::new(navigate_params);
        let navigate_command = NavigateCommand::new(2, navigate);
        let navigate = serde_json::to_value(navigate_command).unwrap();
        // let navigate = navigate_command(2, context_id, "https://www.rust-lang.org/", Some(ReadinessState::Complete));
        bidi_session
            .send_command(navigate)
            .await
            .expect("Failed to send command");

        let navigate_response = bidi_session
            .receive_response()
            .await
            .expect("Failed to receive response");
        println!("Received navigate response: {}", navigate_response);
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Step 5: Close the session
    close_session(base_url, &session.session_id)
        .await
        .expect("Failed to close WebDriver session");
    println!("Session closed.");
}
