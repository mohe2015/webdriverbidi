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
use crate::local::browsing_context::{GetTreeResult, NavigateResult};
use crate::message_handler;
use crate::remote::browsing_context::{GetTreeParameters, NavigateParameters};
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
    /// Creates a new session instance.
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

    /// Starts the session by connecting to the WebSocket URL.
    ///
    /// This method initializes the session by starting it with the WebDriver server,
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

    /// Closes the session and WebSocket connection.
    ///
    /// This method sends a request to the WebDriver server to close the session.
    pub async fn close(&mut self) -> Result<(), SessionError> {
        session::close_session(&self.base_url, &self.session_id).await?;
        Ok(())
    }

    // --------------------------------------------------

    /// Sends a JSON command to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to be sent, which must implement the `Serialize` trait.
    ///
    /// # Returns
    ///
    /// A result containing the response of type `U` which implements the `DeserializeOwned` trait,
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

    /// Spawns a background task to log incoming messages.
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

    /// Retrieves the browsing context tree.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the `GetTree` command.
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

    /// Navigates to a URL in the browsing context.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the `Navigate` command.
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
}
