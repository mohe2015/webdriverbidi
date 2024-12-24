use futures::{SinkExt, StreamExt};
use serde_json::Value;
use std::error::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

#[derive(Debug)]
pub struct OxibidiSession {
    pub websocket_url: String,
    pub websocket_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl OxibidiSession {
    /// Creates a new session and connects to the WebSocket URL
    pub async fn new(websocket_url: String) -> Result<Self, Box<dyn Error>> {
        let (stream, _) = connect_async(&websocket_url).await?;
        println!("Connected to WebSocket: {}", websocket_url);
        Ok(Self {
            websocket_url,
            websocket_stream: Some(stream),
        })
    }

    /// Sends a JSON command to the WebSocket
    pub async fn send_command(&mut self, command: Value) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut stream) = self.websocket_stream {
            let message = Message::Text(command.to_string().into());
            stream.send(message).await?;
            Ok(())
        } else {
            Err("WebSocket stream is not connected".into())
        }
    }

    /// Receives a response from the WebSocket
    pub async fn receive_response(&mut self) -> Result<Value, Box<dyn Error>> {
        if let Some(ref mut stream) = self.websocket_stream {
            if let Some(Ok(Message::Text(response))) = stream.next().await {
                let json: Value = serde_json::from_str(&response)?;
                Ok(json)
            } else {
                Err("Failed to receive response".into())
            }
        } else {
            Err("WebSocket stream is not connected".into())
        }
    }
}
