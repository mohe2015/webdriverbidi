use std::collections::HashMap;
use std::sync::Arc;

// --------------------------------------------------

use futures::SinkExt;
use log::{debug, error};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};
use tokio::time::{timeout, Duration};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

// --------------------------------------------------

use super::error::CommandError;

// --------------------------------------------------

const COMMAND_ID_KEY: &str = "id";
const RESULT_KEY: &str = "result";
// Wait 60 seconds max for a command response
const RECEIVER_TIMEOUT: u64 = 60;

// --------------------------------------------------

/// Sends a command over a WebSocket connection and awaits a response.
///
/// This function serializes the given command, sends it over the provided WebSocket stream,
/// and waits for a response. Timesout if no response is received within 60 seconds.
pub async fn send_command<T: Serialize, U: DeserializeOwned>(
    websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pending_commands: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    command: T,
) -> Result<U, CommandError> {
    let value = serde_json::to_value(command).map_err(|e| {
        error!("Serialization error: {:?}", e);
        CommandError::SerdeError(e)
    })?;
    debug!("Serialized command: {:?}", value);

    let command_id = value
        .get(COMMAND_ID_KEY)
        .and_then(|id| id.as_u64())
        .ok_or_else(|| {
            error!("Missing command ID in the serialized value: {:?}", value);
            CommandError::MissingCommandId
        })?;

    let message = Message::Text(value.to_string().into());

    let (sender, receiver) = oneshot::channel();
    {
        debug!("Locking the pending commands mutex");
        let mut pending_commands = pending_commands.lock().await;
        debug!("Inserting the command");
        pending_commands.insert(command_id, sender);
    }

    {
        debug!("Locking the WebSocket stream mutex");
        let mut websocket_stream = websocket_stream.lock().await;
        if let Err(e) = websocket_stream.send(message).await {
            error!("Error sending message: {:?}", e);
            pending_commands.lock().await.remove(&command_id);
            return Err(CommandError::WebSocketSendError(e));
        }
    }

    debug!("Awaiting a response for command id: {}", command_id);

    // Await the receiver to get the response with a timeout
    let response = timeout(Duration::from_secs(RECEIVER_TIMEOUT), receiver)
        .await
        .map_err(|e| {
            error!("Timeout waiting for response: {:?}", e);
            CommandError::TimeoutError
        })?
        .map_err(|e| {
            error!("Receiver error: {:?}:", e);
            CommandError::OneshotReceiverError(e)
        })?;

    debug!("Received response: {:?}", response);

    let rslt = response.get(RESULT_KEY).ok_or_else(|| {
        error!("Missing result in the response: {:?}", response);
        CommandError::MissingResult
    })?;
    let rslt = serde_json::from_value(rslt.to_owned()).map_err(|e| {
        error!("Deserialization error: {:?} for JSON: {:?}", e, rslt);
        CommandError::SerdeError(e)
    })?;
    Ok(rslt)
}
