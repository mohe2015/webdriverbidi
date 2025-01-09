use tokio;

// --------------------------------------------------

use webdriverbidi::remote::browsing_context::{
    ActivateParameters, CreateParameters, CreateType, GetTreeParameters,
};
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::webdriver::capabilities::CapabilitiesRequest;

// --------------------------------------------------

mod utils;
use utils::sleep;

// --------------------------------------------------

const HOST: &str = "localhost";
const PORT: u16 = 4444;

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-activate

#[tokio::test]
async fn test_browsing_context_activate() {
    // Initialize a new WebDriver BiDi session and start it
    let capabilities = CapabilitiesRequest::default();
    let mut session = WebDriverBiDiSession::new(HOST.into(), PORT, capabilities);
    session.start().await.expect("Failed to start session");

    // Get the browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to get tree");

    let root_context = get_tree_rslt.contexts[0].context.clone();

    sleep(2).await;

    // Add a new tab
    let create_params = CreateParameters::new(CreateType::Tab, None, None, None);
    session
        .browsing_context_create(create_params)
        .await
        .expect("Failed to add a new tab");

    sleep(2).await;

    // Activate the first tab
    let activate_params = ActivateParameters::new(root_context);
    session
        .browsing_context_activate(activate_params)
        .await
        .expect("Failed to activate the first tab");

    // TODO - Is there a way to programmatically verify that the tab was activated?
    sleep(2).await;

    // Close the session
    session.close().await.expect("Failed to close session");
}
