use tokio;
use webdriverbidi::webdriver::capabilities::{Capabilities, CapabilityRequest};
use webdriverbidi::session::WebDriverBiDiSession;

use webdriverbidi::models::remote::browsing_context::{
    NavigateParameters, ReadinessState,
};
use webdriverbidi::models::remote::browsing_context::GetTreeParameters;

#[tokio::main]
async fn main() {

    let always_match = CapabilityRequest::new();  
    let capabilities = Capabilities::new(always_match);

    let mut bidi_session = WebDriverBiDiSession::new("localhost".to_string(), 4444, capabilities)
        .await
        .expect("Failed to connect to WebSocket");

    let get_tree_params = GetTreeParameters::new(None, None);
    
    let get_tree_rslt =
        bidi_session
            .browsing_context_get_tree(get_tree_params)
            .await
            .expect("Failed to send command");
  
    // println!("Received getTree response 1: {:?}", get_tree_rslt);



    
    let context_id = get_tree_rslt.contexts[0].context.clone();
    
    let navigate_params = NavigateParameters::new(
        context_id,
        "https://www.rust-lang.org/".to_string(),
        Some(ReadinessState::Complete),
    );
    
    let navigate_rslt = bidi_session
        .browsing_context_navigate(navigate_params)
        .await
        .expect("Failed to send command");
    
    println!("Received navigate response: {:?}", navigate_rslt);
    
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
    bidi_session.close().await.expect("Failed to close session");
}