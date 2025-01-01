use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};

// --------------------------------------------------

use crate::error::SessionError;
use crate::webdriver::capabilities::Capabilities;

// --------------------------------------------------

/// Represents the information returned when starting a WebDriver session.
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub session_id: String,
    pub capabilities: Capabilities,
    pub websocket_url: String,
}

// --------------------------------------------------

/// Starts a WebDriver session through HTTP.
///
/// # Arguments
///
/// * `base_url` - The base URL of the WebDriver server.
/// * `capabilities` - The features that the session is expected to support.
///
/// # Returns
///
/// A `Result` containing either a SessionResponse instance or a SessionError.
pub async fn start_session(
    base_url: &str,
    capabilities: &Capabilities,
) -> Result<SessionResponse, SessionError> {
    let url = format!("{}/session", base_url);
    let payload = capabilities.build();
    let client = create_http_client();

    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to send HTTP request: {}", e);
            SessionError::HttpRequestError(e)
        })?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| {
            error!("Failed to parse JSON response: {}", e);
            SessionError::HttpRequestError(e)
        })?;

    // Extract sessionId and WebSocket URL
    let session_id = response["value"]["sessionId"].as_str().ok_or_else(|| {
        let msg = format!("JSON doesn't contain a sessionId field: {:?}", response);
        error!("{}", msg);
        SessionError::SessionResponseError(msg)
    })?;
    let websocket_url = response["value"]["capabilities"]["webSocketUrl"]
        .as_str()
        .ok_or_else(|| {
            let msg = format!("JSON doesn't contain a webSocketUrl field: {:?}", response);
            error!("{}", msg);
            SessionError::SessionResponseError(msg)
        })?;

    let session_response = SessionResponse {
        session_id: session_id.to_string(),
        capabilities: serde_json::from_value(response["value"]["capabilities"].clone()).map_err(
            |e| {
                let msg = format!("Failed to deserialize capabilities: {}", e);
                error!("{}", msg);
                SessionError::SessionResponseError(msg)
            },
        )?,
        websocket_url: websocket_url.to_string(),
    };

    debug!("Session started successfully: {:?}", session_response);
    Ok(session_response)
}

// --------------------------------------------------

/// Closes a WebDriver session.
///
/// # Arguments
///
/// * `base_url` - The base URL of the WebDriver server.
/// * `session_id` - The ID of the WebDriver session to close.
///
/// # Returns
///
/// A `Result` containing either `()` or a SessionError.
pub async fn close_session(base_url: &str, session_id: &str) -> Result<(), SessionError> {
    let url = format!("{}/session/{}", base_url, session_id);
    let client = create_http_client();

    client.delete(&url).send().await.map_err(|e| {
        error!("Failed to send HTTP request: {}", e);
        SessionError::HttpRequestError(e)
    })?;

    debug!("Session {} closed successfully", session_id);
    Ok(())
}

// --------------------------------------------------

/// Creates a new reqwest HTTP client.
fn create_http_client() -> Client {
    Client::new()
}
