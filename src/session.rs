use futures::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, oneshot};
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use tokio::task;

#[derive(Debug)]
pub struct WebDriverBiDiSession {
    pub websocket_url: String,
    pub websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pub pending_commands: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
}

impl WebDriverBiDiSession {
    /// Creates a new session and connects to the WebSocket URL
    pub async fn new(websocket_url: String) -> Result<Self, Box<dyn Error>> {
        let (stream, _) = connect_async(&websocket_url).await?;
        println!("Connected to WebSocket: {}", websocket_url);

        let websocket_stream = Arc::new(Mutex::new(stream));
        let pending_commands = Arc::new(Mutex::new(HashMap::new()));

        let session = Self {
            websocket_url,
            websocket_stream: websocket_stream.clone(),
            pending_commands: pending_commands.clone(),
        };

        // Spawn a background task to log incoming messages
        session.spawn_message_handler_task(websocket_stream, pending_commands);

        Ok(session)
    }

    /// Sends a JSON command to the WebSocket
    pub async fn send_command<T: Serialize, U: DeserializeOwned>(&mut self, command: T) -> Result<U, Box<dyn Error>> {
        let value = serde_json::to_value(command)?;
        let command_id = value.get("id")
            .and_then(|id| id.as_u64())
            .ok_or("Command must have an 'id' field")?;

        let message = Message::Text(value.to_string().into());

        let (sender, receiver) = oneshot::channel();
        self.pending_commands.lock().await.insert(command_id, sender);
        
        {
            let mut websocket_stream = self.websocket_stream.lock().await;
            websocket_stream.send(message).await?;
        }
        
        // Await the receiver to get the response
        let response = receiver.await?;
        println!("Received response inside send_command: {}", response);
        let rslt = response["result"].clone();
        println!("Received result inside send_command: {}", rslt);
        let u = serde_json::from_value(rslt)?;
        Ok(u)
    }

    /// Receives a response from the WebSocket
    // pub async fn receive_response(&mut self) -> Result<Value, Box<dyn Error>> {
    //     let mut websocket_stream = self.websocket_stream.lock().await;
    //     if let Some(Ok(Message::Text(response))) = websocket_stream.next().await {
    //         let json: Value = serde_json::from_str(&response)?;
    //         if let Some(id) = json.get("id").and_then(|id| id.as_u64()) {
    //             if let Some(sender) = self.pending_commands.lock().await.remove(&id) {
    //                 let _ = sender.send(json.clone());
    //             }
    //         }
    //         Ok(json)
    //     } else {
    //         Err("Failed to receive response".into())
    //     }
    // }

    /// Spawns a background task to log incoming messages
    fn spawn_message_handler_task(
        &self,
        websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
        pending_commands: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
    ) {
        task::spawn(async move {
            let mut websocket_stream = websocket_stream.lock().await;
            while let Some(message) = websocket_stream.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        println!("Received message: {}", text);
                        let json: Value = serde_json::from_str(&text).unwrap();
                        if let Some(id) = json.get("id").and_then(|id| id.as_u64()) {
                            if let Some(sender) = pending_commands.lock().await.remove(&id) {
                                let _ = sender.send(json);
                            }
                        }
                        else {
                            eprintln!("Received message without an 'id' field: {}", text);
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error receiving message: {}", e);
                        break;
                    }
                }
            }
        });
    }
}