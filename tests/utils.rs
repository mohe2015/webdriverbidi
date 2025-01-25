use base64::prelude::*;
// use ctor::ctor;
// use simplelog::*;
use std::fs::File;
use std::io::Write;
use tokio::time;

// --------------------------------------------------

use webdriverbidi::remote::browsing_context::{
    CreateParameters, CreateType, GetTreeParameters, NavigateParameters, ReadinessState,
};
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::webdriver::capabilities::CapabilitiesRequest;

// --------------------------------------------------

const HOST: &str = "localhost";
const PORT: u16 = 4444;

// --------------------------------------------------

/// Sleep for a given number of seconds.
pub async fn sleep(secs: u64) {
    time::sleep(time::Duration::from_secs(secs)).await
}

// --------------------------------------------------

/// Save a Base64-encoded screenshot to a file.
pub fn save_screenshot(base64_data: &str, file_path: &str) -> std::io::Result<()> {
    // Decode the Base64 string into bytes
    let decoded_data = BASE64_STANDARD
        .decode(base64_data)
        .expect("Failed to decode Base64 data");

    // Create a new file and write the decoded bytes
    let mut file = File::create(file_path)?;
    file.write_all(&decoded_data)?;

    println!("Screenshot saved to {}", file_path);
    Ok(())
}

// --------------------------------------------------

/// Initialize a new WebDriver BiDi session and start it.
pub async fn init_session() -> WebDriverBiDiSession {
    let capabilities = CapabilitiesRequest::default();
    let mut session = WebDriverBiDiSession::new(HOST.into(), PORT, capabilities);
    session.start().await.expect("Failed to start session");
    session
}

// --------------------------------------------------

/// Close the WebDriver BiDi session.
pub async fn close_session(session: &mut WebDriverBiDiSession) {
    session.close().await.expect("Failed to close session");
}

// --------------------------------------------------

/// Get the first browsing context from the browsing context tree.
pub async fn get_first_context(
    session: &mut WebDriverBiDiSession,
) -> Result<String, Box<dyn std::error::Error>> {
    let get_tree_params = GetTreeParameters::new(None, None);
    let get_tree_rslt = session.browsing_context_get_tree(get_tree_params).await?;
    Ok(get_tree_rslt.contexts[0].context.clone())
}

// --------------------------------------------------

/// Open a new tab.
pub async fn new_tab(session: &mut WebDriverBiDiSession) {
    let create_params = CreateParameters::new(CreateType::Tab, None, None, None);
    session
        .browsing_context_create(create_params)
        .await
        .expect("Failed to open a new tab");
}

// --------------------------------------------------

// /// Initialize a simplelog TermLogger.
// #[ctor]
// fn init() {
//     TermLogger::init(
//         LevelFilter::Debug,
//         Config::default(),
//         TerminalMode::Mixed,
//         ColorChoice::Auto,
//     )
//     .unwrap();
// }

// --------------------------------------------------

/// Navigates to the specified URL waiting for the document to completely load.
pub async fn navigate(context: String, url: String, session: &mut WebDriverBiDiSession) {
    let navigate_params =
        NavigateParameters::new(context.clone(), url, Some(ReadinessState::Complete));
    session
        .browsing_context_navigate(navigate_params)
        .await
        .expect("Failed to navigate");
}
