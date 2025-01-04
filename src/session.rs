use log::debug;
use std::collections::HashMap;
use std::sync::Arc;

// --------------------------------------------------

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};
use tokio::task;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

// --------------------------------------------------

use crate::command_sender;
use crate::commands;
use crate::error::{CommandError, SessionError};
use crate::local::browsing_context::*;
use crate::message_handler;
use crate::models::local::result_data::EmptyResult;
use crate::remote::browsing_context::*;
use crate::webdriver::capabilities::Capabilities;
use crate::webdriver::session;

// --------------------------------------------------

/// Represents a WebDriver BiDi session.
///
/// This struct manages the lifecycle of a WebDriver session, including
/// starting the session, establishing a WebSocket connection, sending
/// commands, handling incoming messages and closing it.
///
/// # Fields
///
/// * `host` - The host address of the WebDriver server.
/// * `port` - The port number of the WebDriver server.
/// * `base_url` - The base URL constructed from the host and port.
/// * `session_id` - The unique identifier for the session.
/// * `capabilities` - The desired capabilities for the session.
/// * `websocket_url` - The WebSocket URL for bidirectional communication.
/// * `websocket_stream` - The WebSocket stream for communication protected by an `Arc` wrapped `Mutex`.
/// * `pending_commands` - A map of pending commands awaiting responses protected by an `Arc` wrapped `Mutex`.
#[derive(Debug)]
pub struct WebDriverBiDiSession {
    pub host: String,
    pub port: u16,
    pub base_url: String,
    pub session_id: String,
    pub capabilities: Capabilities,
    pub websocket_url: String,
    pub websocket_stream: Option<Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    pub pending_commands: Option<Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>>,
}

// --------------------------------------------------

impl WebDriverBiDiSession {
    /// Creates a new session.
    ///
    /// # Arguments
    ///
    /// * `host` - The host address of the WebDriver server.
    /// * `port` - The port number of the WebDriver server.
    /// * `capabilities` - The desired capabilities for the session.
    pub fn new(host: String, port: u16, capabilities: Capabilities) -> Self {
        let base_url = format!("http://{}:{}", host, port);
        debug!("Constructed base URL: {}", base_url);
        Self {
            host,
            port,
            base_url,
            session_id: String::new(),
            capabilities,
            websocket_url: String::new(),
            websocket_stream: None,
            pending_commands: None,
        }
    }

    // --------------------------------------------------

    /// Initializes the session by starting it with the WebDriver server,
    /// establishing a WebSocket connection, and spawning a background task to handle
    /// incoming messages.
    ///
    /// **A WebDriver BiDi server must be running before calling this method.**
    pub async fn start(&mut self) -> Result<(), SessionError> {
        let session = session::start_session(&self.base_url, &self.capabilities)
            .await
            .map_err(|e| SessionError::Other(format!("Failed to start session: {}", e)))?;
        self.session_id = session.session_id;
        self.websocket_url = session.websocket_url;

        debug!("Establishing the WebSocket connection");
        let (stream, _) = connect_async(&self.websocket_url)
            .await
            .map_err(|e| SessionError::Other(format!("Failed to connect to WebSocket: {}", e)))?;

        let websocket_stream = Arc::new(Mutex::new(stream));
        let pending_commands = Arc::new(Mutex::new(HashMap::new()));

        self.websocket_stream = Some(websocket_stream.clone());
        self.pending_commands = Some(pending_commands.clone());

        debug!("Starting the incoming messages manager loop");
        // Spawn a background task to manage incoming messages
        self.spawn_message_handler_task(websocket_stream, pending_commands);

        Ok(())
    }

    // --------------------------------------------------

    /// Sends a request to the WebDriver server to close the session.
    pub async fn close(&mut self) -> Result<(), SessionError> {
        session::close_session(&self.base_url, &self.session_id).await?;
        Ok(())
    }

    // --------------------------------------------------

    /// Sends a WebDriver BiDi command.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to send.
    ///
    /// # Returns
    ///
    /// A result containing the response of type `U` that implements the `DeserializeOwned` trait,
    /// or a `CommandError` if the command could not be sent.
    pub async fn send_command<T: Serialize, U: DeserializeOwned>(
        &mut self,
        command: T,
    ) -> Result<U, CommandError> {
        if let (Some(websocket_stream), Some(pending_commands)) =
            (&self.websocket_stream, &self.pending_commands)
        {
            command_sender::send_command(
                websocket_stream.clone(),
                pending_commands.clone(),
                command,
            )
            .await
        } else {
            let error_msg = format!("WebSocket stream or pending commands mutex not initialized. WebSocket stream: {}, pending commands: {}", self.websocket_stream.is_some(), self.pending_commands.is_some());
            Err(CommandError::Other(error_msg.into()))
        }
    }

    // --------------------------------------------------

    /// Spawns a background task to manage incoming WebSocket messages.
    ///
    /// This method creates a new asynchronous task that continuously listens for
    /// incoming messages on the WebSocket connection and handles them appropriately.
    fn spawn_message_handler_task(
        &self,
        websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
        pending_commands: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    ) {
        task::spawn(message_handler::handle_messages(
            websocket_stream,
            pending_commands,
        ));
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-activate

    /// Activates and focuses a browsing context.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as an `ActivateParameters` instance.
    ///
    /// # Returns
    ///
    /// A result containing the `EmptyResult` or a `CommandError`.
    pub async fn browsing_context_activate(
        &mut self,
        params: ActivateParameters,
    ) -> Result<EmptyResult, CommandError> {
        commands::browsing_context::activate(self, params).await
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-captureScreenshot

    /// Captures an image of the given navigable, and returns it as a Base64-encoded string.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as a `CaptureScreenshotParameters` instance.
    ///
    /// # Returns
    ///
    /// A result containing the `CaptureScreenshotResult` or a `CommandError`.
    pub async fn browsing_context_capture_screenshot(
        &mut self,
        params: CaptureScreenshotParameters,
    ) -> Result<CaptureScreenshotResult, CommandError> {
        commands::browsing_context::capture_screenshot(self, params).await
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-close

    /// Closes the browsing context.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as a `CloseParameters` instance.
    ///
    /// # Returns
    ///
    /// A result containing the `EmptyResult` or a `CommandError`.
    pub async fn browsing_context_close(
        &mut self,
        params: CloseParameters,
    ) -> Result<EmptyResult, CommandError> {
        commands::browsing_context::close(self, params).await
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-create

    /// Creates a new browsing context.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as a `CreateParameters` instance.
    ///
    /// # Returns
    ///
    /// A result containing the `CreateResult` or a `CommandError`.
    pub async fn browsing_context_create(
        &mut self,
        params: CreateParameters,
    ) -> Result<CreateResult, CommandError> {
        commands::browsing_context::create(self, params).await
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-getTree

    /// Retrieves the browsing context tree.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as a GetTreeParameters instance.
    ///
    /// # Returns
    ///
    /// A result containing the `GetTreeResult` or a `CommandError`.
    pub async fn browsing_context_get_tree(
        &mut self,
        params: GetTreeParameters,
    ) -> Result<GetTreeResult, CommandError> {
        commands::browsing_context::get_tree(self, params).await
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-handleUserPrompt

    /// Allows closing an open prompt.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as a `HandleUserPromptParameters` instance.
    ///
    /// # Returns
    ///
    /// A result containing the `EmptyResult` or a `CommandError`.
    pub async fn browsing_context_handle_user_prompt(
        &mut self,
        params: HandleUserPromptParameters,
    ) -> Result<EmptyResult, CommandError> {
        commands::browsing_context::handle_user_prompt(self, params).await
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-locateNodes

    /// Returns a list of all nodes matching the specified locator.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as a `LocateNodesParameters` instance.
    ///
    /// # Returns
    ///
    /// A result containing the `LocateNodesResult` or a `CommandError`.
    pub async fn browsing_context_locate_nodes(
        &mut self,
        params: LocateNodesParameters,
    ) -> Result<LocateNodesResult, CommandError> {
        commands::browsing_context::locate_nodes(self, params).await
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-navigate

    /// Navigates to a URL in the browsing context.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as a NavigateParameters instance.
    ///
    /// # Returns
    ///
    /// A result containing the `NavigateResult` or a `CommandError`.
    pub async fn browsing_context_navigate(
        &mut self,
        params: NavigateParameters,
    ) -> Result<NavigateResult, CommandError> {
        commands::browsing_context::navigate(self, params).await
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-print

    /// Creates a paginated representation of a document, and returns it
    /// as a PDF document represented as a Base64-encoded string.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as a `PrintParameters` instance.
    ///
    /// # Returns
    ///
    /// A result containing the `PrintResult` or a `CommandError`.
    pub async fn browsing_context_print(
        &mut self,
        params: PrintParameters,
    ) -> Result<PrintResult, CommandError> {
        commands::browsing_context::print(self, params).await
    }

    // --------------------------------------------------

    // https://w3c.github.io/webdriver-bidi/#command-browsingContext-traverseHistory

    /// Navigates through the browsing history of a specified context.
    ///
    /// This method allows you to move forward or backward in the browsing history
    /// of a given navigable context by a specified number of steps (delta).
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters as a `TraverseHistoryParameters` instance, which
    ///   includes the context identifier and the delta indicating the number of steps
    ///   to move in the history. A positive delta moves forward, while a negative delta
    ///   moves backward.
    ///
    /// # Returns
    ///
    /// A result containing the `TraverseHistoryResult` or a `CommandError` if the
    /// operation fails.
    pub async fn browsing_context_traverse_history(
        &mut self,
        params: TraverseHistoryParameters,
    ) -> Result<TraverseHistoryResult, CommandError> {
        commands::browsing_context::traverse_history(self, params).await
    }
}
