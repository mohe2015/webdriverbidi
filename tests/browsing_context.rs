use base64::prelude::*;

// --------------------------------------------------

use webdriverbidi::remote::browsing_context::*;
use webdriverbidi::remote::script::{ContextTarget, EvaluateParameters, Target};

// --------------------------------------------------

mod utils;
use utils::*;

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-activate

#[tokio::test]
async fn test_browsing_context_activate() {
    let mut session = init_session().await;

    // Open a new tab
    new_tab(&mut session).await;
    sleep(1).await;

    // Activate the first tab
    let first_tab_context = get_first_context(&mut session)
        .await
        .expect("Failed to get first browsing context");
    let activate_params = ActivateParameters::new(first_tab_context);
    session
        .browsing_context_activate(activate_params)
        .await
        .expect("Failed to activate the first tab");

    // TODO - Is there a way to programmatically verify that the first tab was activated?
    sleep(1).await;
    close_session(&mut session).await;
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-captureScreenshot

#[tokio::test]
async fn test_browsing_context_capture_screenshot() {
    let mut session = init_session().await;

    // Get the first browsing context
    let context = get_first_context(&mut session)
        .await
        .expect("Failed to get first browsing context");

    // Navigate to rust-lang.org
    let navigate_params = NavigateParameters::new(
        context.clone(),
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

    let rslt = session
        .browsing_context_capture_screenshot(params)
        .await
        .expect("Failed to capture screenshot");

    // Save the screenshot to a file
    if let Err(e) = save_screenshot(rslt.data.as_str(), "screenshot.png") {
        eprintln!("Error saving screenshot: {}", e);
    }

    close_session(&mut session).await;
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-close

#[tokio::test]
async fn test_browsing_context_close() {
    let mut session = init_session().await;

    // Open a new tab, some browsers might not close the browser itself when the last tab or window is closed
    new_tab(&mut session).await;
    sleep(1).await;

    // Close the first tab
    let first_tab_context = get_first_context(&mut session)
        .await
        .expect("Failed to get first browsing context");
    let close_params = CloseParameters::new(first_tab_context, None);
    session
        .browsing_context_close(close_params)
        .await
        .expect("Failed to activate the first tab");

    sleep(1).await;
    close_session(&mut session).await;
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-create

#[tokio::test]
async fn test_browsing_context_create() {
    let mut session = init_session().await;

    // Get the initial browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let initial_tree = session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to get initial browsing context tree");

    let initial_count = initial_tree.contexts.len();

    // Open a new tab
    let create_params = CreateParameters::new(CreateType::Tab, None, None, None);
    let create_result = session
        .browsing_context_create(create_params)
        .await
        .expect("Failed to create a new tab");

    // The ID of the newly created context
    let new_context_id = create_result.context;

    // Get the updated browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let updated_tree = session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to get updated browsing context tree");

    // Verify there's exactly one more context
    let updated_count = updated_tree.contexts.len();
    assert_eq!(
        updated_count,
        initial_count + 1,
        "Expected exactly one new browsing context to be created"
    );

    // Verify the new context's ID appears among the list of contexts
    let found = updated_tree
        .contexts
        .iter()
        .any(|ctx| ctx.context == new_context_id);
    assert!(
        found,
        "New browsing context was not found in the updated tree"
    );

    close_session(&mut session).await;
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-getTree

#[tokio::test]
async fn test_browsing_context_get_tree() {
    let mut session = init_session().await;

    // Get the browsing context tree
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = session
        .browsing_context_get_tree(get_tree_params)
        .await
        .expect("Failed to get tree");

    // Make sure we got at least one context
    assert!(
        !get_tree_rslt.contexts.is_empty(),
        "Expected at least one browsing context"
    );

    close_session(&mut session).await;
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-handleUserPrompt

#[tokio::test]
async fn test_browsing_context_handle_user_prompt() {
    let mut session = init_session().await;

    // Define the HTML content with a script to trigger an alert dialog
    let html_content = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Test Page</title>
            <script type="text/javascript">
                function triggerAlert() {
                    alert('This is a test alert');
                }
            </script>
        </head>
        <body>
            <h1>Hello, WebDriver BiDi!</h1>
            <button onclick="triggerAlert()">Trigger Alert</button>
        </body>
        </html>
    "#;

    // Create a data URL with the HTML content
    let data_url = format!(
        "data:text/html;base64,{}",
        BASE64_STANDARD.encode(html_content)
    );

    // Load the data URL
    let context = get_first_context(&mut session)
        .await
        .expect("Failed to get first browsing context");
    let navigate_params =
        NavigateParameters::new(context.clone(), data_url, Some(ReadinessState::Complete));
    session
        .browsing_context_navigate(navigate_params)
        .await
        .expect("Failed to load the data URL");

    // Trigger the alert dialog asynchronously
    // triggerAlert(); alone prevents receiving the command's response
    let script = r#"
        setTimeout(() => {
            triggerAlert();
        }, 100);
    "#;
    let evaluate_params = EvaluateParameters::new(
        script.to_string(),
        Target::ContextTarget(ContextTarget::new(context.clone(), None)),
        false,
        None,
        None,
        None,
    );
    session
        .script_evaluate(evaluate_params)
        .await
        .expect("Failed to evaluate script");

    sleep(1).await;

    // Close the alert dialog
    let handle_prompt_params = HandleUserPromptParameters::new(context, Some(true), None);
    session
        .browsing_context_handle_user_prompt(handle_prompt_params)
        .await
        .expect("Failed to handle user prompt");

    sleep(1).await;
    close_session(&mut session).await;
}

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-locateNodes
// Not yet implemented in Chrome, Firefox, or Edge
// Last checked: 2025-01-18

// --------------------------------------------------

// https://w3c.github.io/webdriver-bidi/#command-browsingContext-navigate

#[tokio::test]
async fn test_browsing_context_navigate() {
    let mut session = init_session().await;

    // Get the first browsing context
    let context = get_first_context(&mut session)
        .await
        .expect("Failed to get first browsing context");

    // Navigate to rust-lang.org
    let navigate_params = NavigateParameters::new(
        context.clone(),
        "https://www.rust-lang.org/".to_string(),
        Some(ReadinessState::Complete),
    );

    // Verify the navigation result URL
    let nav_rslt = session
        .browsing_context_navigate(navigate_params)
        .await
        .expect("Failed to navigate");
    assert_eq!(nav_rslt.url, "https://www.rust-lang.org/");

    close_session(&mut session).await;
}
