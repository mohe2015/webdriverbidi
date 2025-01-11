use tokio;

// --------------------------------------------------

use webdriverbidi::remote::browsing_context::*;

// --------------------------------------------------

mod utils;
use utils::{close_session, init_session, save_screenshot, sleep};

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-activate

#[tokio::test]
async fn test_browsing_context_activate() {
    let mut session = init_session().await;

    // Get the browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to get tree");

    let first_tab_context = get_tree_rslt.contexts[0].context.clone();

    sleep(2).await;

    // Add a new tab
    let create_params = CreateParameters::new(CreateType::Tab, None, None, None);
    session
        .browsing_context_create(create_params)
        .await
        .expect("Failed to add a new tab");

    sleep(2).await;

    // Activate the first tab
    let activate_params = ActivateParameters::new(first_tab_context);
    session
        .browsing_context_activate(activate_params)
        .await
        .expect("Failed to activate the first tab");

    // TODO - Is there a way to programmatically verify that the tab was activated?
    sleep(2).await;

    close_session(&mut session).await;
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-captureScreenshot

#[tokio::test]
async fn test_browsing_context_capture_screenshot() {
    let mut session = init_session().await;

    // Get the browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to get tree");

    let context = get_tree_rslt.contexts[0].context.clone();

    // Navigate to rust-lang.org
    let navigate_params = NavigateParameters::new(
        get_tree_rslt.contexts[0].context.clone(),
        "https://www.rust-lang.org/".to_string(),
        Some(ReadinessState::Complete),
    );
    session
        .browsing_context_navigate(navigate_params)
        .await
        .expect("Failed to navigate");

    // Capture a screenshot
    let params = CaptureScreenshotParameters {
        context,
        origin: Some(CaptureScreenshotParametersOrigin::Document),
        format: Some(ImageFormat {
            image_format_type: "png".to_owned(),
            quality: None,
        }),
        clip: None,
    };

    let png = session
        .browsing_context_capture_screenshot(params)
        .await
        .expect("Failed to capture screenshot");

    // Save the screenshot to a file
    if let Err(e) = save_screenshot(png.data.as_str(), "screenshot.png") {
        eprintln!("Error saving screenshot: {}", e);
    }

    close_session(&mut session).await;
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-close

#[tokio::test]
async fn test_browsing_context_close() {
    let mut session = init_session().await;

    // Get the browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to get tree");

    let first_tab_context = get_tree_rslt.contexts[0].context.clone();

    sleep(2).await;

    // Add a new tab, some browsers might not close the browser itself when the last tab or window is closed
    let create_params = CreateParameters::new(CreateType::Tab, None, None, None);
    session
        .browsing_context_create(create_params)
        .await
        .expect("Failed to add a new tab");

    sleep(2).await;

    // Close the first tab
    let close_params = CloseParameters::new(first_tab_context, None);
    session
        .browsing_context_close(close_params)
        .await
        .expect("Failed to activate the first tab");

    sleep(2).await;

    close_session(&mut session).await;
}
