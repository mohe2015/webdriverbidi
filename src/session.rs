use crate::command_sender;
use crate::message_handler;
use crate::models::local::browsing_context::GetTreeResult;
use crate::webdriver::capabilities::Capabilities;
use crate::webdriver::session::{close_session, start_session};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};
use tokio::task;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use crate::error::CommandError;

#[derive(Debug)]
pub struct WebDriverBiDiSession {
    pub host: String,
    pub port: u16,
    pub base_url: String,
    pub session_id: String,
    pub capabilities: Value,
    pub websocket_url: String,
    pub websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pub pending_commands: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
}

impl WebDriverBiDiSession {
    /// Creates a new session and connects to the WebSocket URL
    pub async fn new(
        host: String,
        port: u16,
        capabilities: Capabilities,
    ) -> Result<Self, Box<dyn Error>> {
        let base_url = format!("http://{}:{}", host, port);
        let session = start_session(&base_url, capabilities).await?;
        let websocket_url = session.websocket_url;
        let (stream, _) = connect_async(&websocket_url).await?;
        println!("Connected to WebSocket: {}", websocket_url);

        let websocket_stream = Arc::new(Mutex::new(stream));
        let pending_commands = Arc::new(Mutex::new(HashMap::new()));

        let session = Self {
            host,
            port,
            base_url,
            session_id: session.session_id,
            capabilities: session.capabilities,
            websocket_url,
            websocket_stream: websocket_stream.clone(),
            pending_commands: pending_commands.clone(),
        };

        // Spawn a background task to manage incoming messages
        session.spawn_message_handler_task(websocket_stream, pending_commands);

        Ok(session)
    }

    /// Closes the session and WebSocket connection
    pub async fn close(&mut self) -> Result<(), Box<dyn Error>> {
        close_session(&self.base_url, &self.session_id).await?;
        Ok(())
    }

    /// Sends a JSON command to the WebSocket
    pub async fn send_command<T: Serialize, U: DeserializeOwned>(
        &mut self,
        command: T,
    ) -> Result<U, CommandError> {
        command_sender::send_command(
            self.websocket_stream.clone(),
            self.pending_commands.clone(),
            command,
        )
        .await
    }

    /// Spawns a background task to log incoming messages
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

    pub async fn browsing_context_get_tree(
        &mut self,
        params: crate::models::remote::browsing_context::GetTreeParameters,
    ) -> Result<GetTreeResult, CommandError> {
        crate::commands::browsing_context::get_tree(self, params).await
    }

    pub async fn browsing_context_navigate(
        &mut self,
        params: crate::models::remote::browsing_context::NavigateParameters,
    ) -> Result<crate::models::local::browsing_context::NavigateResult, CommandError> {
        crate::commands::browsing_context::navigate(self, params).await
    }
}
