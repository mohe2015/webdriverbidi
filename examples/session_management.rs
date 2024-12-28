use tokio;
use webdriverbidi::webdriver::capabilities::{Capabilities, CapabilityRequest};
use webdriverbidi::webdriver::session::{close_session, start_session};
use webdriverbidi::session::WebDriverBiDiSession;

use webdriverbidi::commands::browsing_context::NavigateCommand;
use webdriverbidi::models::remote::browsing_context::{
    Navigate, NavigateParameters, ReadinessState,
};
use webdriverbidi::commands::browsing_context::GetTreeCommand;
use webdriverbidi::models::remote::browsing_context::{GetTree, GetTreeParameters};
use webdriverbidi::models::local::browsing_context::GetTreeResult;

#[tokio::main]
async fn main() {
    // Base URL of the WebDriver server (GeckoDriver, ChromeDriver or MSEdgeDriver)
    let base_url = "http://localhost:4444";

    let always_match = CapabilityRequest::new();
    
    let capabilities = Capabilities::new(always_match);

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
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_cmd = GetTreeCommand::new(GetTree::new(get_tree_params));
    
    let recv : GetTreeResult =
        bidi_session
            .send_command::<GetTreeCommand, GetTreeResult>(get_tree_cmd)
            .await
            .expect("Failed to send command");

    // let resp = recv.await.expect("Failed to receive response");
    
    println!("Received getTree response: {:?}", recv);
    // let response = bidi_session
    //     .receive_response()
    //     .await
    //     .expect("Failed to receive response");
    // println!("Received getTree response: {}", response);

    // // Step 4: Extract browsingContextId and navigate
    // if let Some(context_id) = response["result"]["contexts"][0]["context"].as_str() {
    //     let navigate_params = NavigateParameters::new(
    //         context_id.to_string(),
    //         "https://www.rust-lang.org/".to_string(),
    //         Some(ReadinessState::Complete),
    //     );
    //     let navigate_command = NavigateCommand::new(Navigate::new(navigate_params));
    //     bidi_session
    //         .send_command(navigate_command)
    //         .await
    //         .expect("Failed to send command");

    //     let navigate_response = bidi_session
    //         .receive_response()
    //         .await
    //         .expect("Failed to receive response");
    //     println!("Received navigate response: {}", navigate_response);
    // }

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Step 5: Close the session
    close_session(base_url, &session.session_id)
        .await
        .expect("Failed to close WebDriver session");
    println!("Session closed.");
}