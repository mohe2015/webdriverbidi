use super::error::CommandError;
use futures::SinkExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;

const COMMAND_ID_KEY: &str = "id";
const RESULT_KEY: &str = "result";

pub async fn send_command<T: Serialize, U: DeserializeOwned>(
    websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pending_commands: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    command: T,
) -> Result<U, CommandError> {
    let value = serde_json::to_value(command).map_err(|e| CommandError::SerializationError(e))?;

    let command_id = value
        .get(COMMAND_ID_KEY)
        .and_then(|id| id.as_u64())
        .ok_or(CommandError::MissingCommandId)?;

    let message = Message::Text(value.to_string().into());

    let (sender, receiver) = oneshot::channel();
    {
        let mut pending_commands = pending_commands.lock().await;
        pending_commands.insert(command_id, sender);
    }

    {
        let mut websocket_stream = websocket_stream.lock().await;
        if let Err(e) = websocket_stream.send(message).await {
            eprintln!("Error sending message: {:?}", e);
            pending_commands.lock().await.remove(&command_id);
            return Err(CommandError::WebSocketSendError(e));
        }
    }

    // Await the receiver to get the response
    let response = receiver
        .await
        .map_err(|e| CommandError::OneshotReceiverError(e))?;
    let rslt = response
        .get(RESULT_KEY)
        .ok_or(CommandError::MissingResult)?
        .clone();
    let rslt = serde_json::from_value(rslt).map_err(|e| CommandError::SerializationError(e))?;
    Ok(rslt)
}
