use anyhow::Result;
// use base64::prelude::*;
// use ctor::ctor;
// use simplelog::*;
// use std::fs::File;
// use std::io::Write;
// use tokio::time;

// --------------------------------------------------

// use webdriverbidi::remote::browsing_context::{
//     CreateParameters, CreateType, GetTreeParameters, NavigateParameters, ReadinessState,
//     TraverseHistoryParameters,
// };
use webdriverbidi::remote::EmptyParams;
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::webdriver::capabilities::CapabilitiesRequest;

// --------------------------------------------------

const HOST: &str = "localhost";
const PORT: u16 = 4444;

// --------------------------------------------------

/// Initializes a new WebDriver BiDi.
pub async fn init_session() -> Result<WebDriverBiDiSession> {
    let capabilities = CapabilitiesRequest::default();
    let mut session = WebDriverBiDiSession::new(HOST.into(), PORT, capabilities);
    session.start().await?;
    Ok(session)
}

/// Close the WebDriver BiDi session.
pub async fn close_session(session: &mut WebDriverBiDiSession) -> Result<()> {
    session.close().await?;
    Ok(())
}

// --------------------------------------------------

/// Returns the list of string Ids of the current user contexts.
pub async fn get_user_context_ids(bidi_session: &mut WebDriverBiDiSession) -> Result<Vec<String>> {
    let user_contexts = bidi_session
        .browser_get_user_contexts(EmptyParams::new())
        .await?
        .user_contexts;

    let user_contexts = user_contexts
        .into_iter()
        .map(|user_context_info| user_context_info.user_context)
        .collect::<Vec<String>>();

    Ok(user_contexts)
}

// --------------------------------------------------

// /// Sleep for a given number of seconds.
// pub async fn sleep_for_secs(secs: u64) {
//     time::sleep(time::Duration::from_secs(secs)).await
// }

// // --------------------------------------------------

// /// Save a Base64-encoded screenshot to a file.
// pub fn save_screenshot(base64_data: &str, file_path: &str) -> std::io::Result<()> {
//     // Decode the Base64 string into bytes
//     let decoded_data = BASE64_STANDARD
//         .decode(base64_data)
//         .expect("Failed to decode Base64 data");

//     // Create a new file and write the decoded bytes
//     let mut file = File::create(file_path)?;
//     file.write_all(&decoded_data)?;

//     println!("Screenshot saved to {}", file_path);
//     Ok(())
// }

// // --------------------------------------------------

// /// Get the first browsing context from the browsing context tree.
// pub async fn get_first_context(
//     session: &mut WebDriverBiDiSession,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let get_tree_params = GetTreeParameters::new(None, None);
//     let get_tree_rslt = session.browsing_context_get_tree(get_tree_params).await?;
//     Ok(get_tree_rslt.contexts[0].context.clone())
// }

// // --------------------------------------------------

// /// Open a new tab.
// pub async fn new_tab(session: &mut WebDriverBiDiSession) {
//     let create_params = CreateParameters::new(CreateType::Tab, None, None, None);
//     session
//         .browsing_context_create(create_params)
//         .await
//         .expect("Failed to open a new tab");
// }

// // --------------------------------------------------

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

// // --------------------------------------------------

// /// Navigates to the specified URL waiting for the document to completely load.
// pub async fn navigate(context: String, url: String, session: &mut WebDriverBiDiSession) {
//     let navigate_params =
//         NavigateParameters::new(context.clone(), url, Some(ReadinessState::Complete));
//     session
//         .browsing_context_navigate(navigate_params)
//         .await
//         .expect("Failed to navigate");
// }

// // --------------------------------------------------

// pub async fn traverse_history(session: &mut WebDriverBiDiSession, context: String, delta: i64) {
//     let traverse_history_params = TraverseHistoryParameters::new(context.clone(), delta);
//     session
//         .browsing_context_traverse_history(traverse_history_params)
//         .await
//         .expect("Failed to send command");
// }
