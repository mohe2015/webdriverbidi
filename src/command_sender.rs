use futures::SinkExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, oneshot};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;

pub async fn send_command<T: Serialize, U: DeserializeOwned>(
    websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pending_commands: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    command: T,
) -> Result<U, Box<dyn Error>> {
    let value = serde_json::to_value(command)?;
    // println!("Sending command: {:?}", value);
    let command_id = value.get("id")
        .and_then(|id| id.as_u64())
        .ok_or("Command must have an 'id' field")?;

    let message = Message::Text(value.to_string().into());

    let (sender, receiver) = oneshot::channel();
    {
        // println!("Acquiring lock for pending_commands inside send_command");
        let mut pending_commands = pending_commands.lock().await;
        // println!("Lock acquired for pending_commands inside send_command");
        pending_commands.insert(command_id, sender);
    }

    {
        // println!("Acquiring lock for websocket_stream inside send_command");
        let mut websocket_stream = websocket_stream.lock().await;
        // println!("Lock acquired for websocket_stream inside send_command");
        if let Err(e) = websocket_stream.send(message).await {
            eprintln!("Error sending message: {:?}", e);
            pending_commands.lock().await.remove(&command_id);
            return Err(Box::new(e));
        }
    }

    // println!("Message sent");

    // Await the receiver to get the response
    let response = receiver.await?;
    // println!("Received response inside send_command: {}", response);
    let rslt = response["result"].clone();
    // println!("Received result inside send_command: {}", rslt);
    let u = serde_json::from_value(rslt)?;
    Ok(u)
}