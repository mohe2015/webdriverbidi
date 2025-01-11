use base64::prelude::*;
use std::fs::File;
use std::io::Write;
use tokio::time;

// --------------------------------------------------

use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::webdriver::capabilities::CapabilitiesRequest;

// --------------------------------------------------

const HOST: &str = "localhost";
const PORT: u16 = 4444;

// --------------------------------------------------

pub async fn sleep(secs: u64) {
    time::sleep(time::Duration::from_secs(secs)).await
}

// --------------------------------------------------

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

pub async fn close_session(session: &mut WebDriverBiDiSession) {
    session.close().await.expect("Failed to close session");
}
