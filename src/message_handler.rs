
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;

use futures::stream::Stream;
use futures::task::{Context, Poll};
use std::pin::Pin;

pub async fn handle_messages(
    websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pending_commands: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
) {
    loop {
        let message = {
            let mut websocket_stream = websocket_stream.lock().await;

            let waker = futures::task::noop_waker();
            let mut cx = Context::from_waker(&waker);

            match Pin::new(&mut *websocket_stream).poll_next(&mut cx) {
                Poll::Ready(Some(Ok(msg))) => Some(Ok(msg)),
                Poll::Ready(Some(Err(e))) => Some(Err(e)),
                Poll::Ready(None) => None,
                Poll::Pending => None,
            }
        };

        match message {
            Some(Ok(Message::Text(text))) => {
                let json: Value = serde_json::from_str(&text).unwrap();
                if let Some(id) = json.get("id").and_then(|id| id.as_u64()) {
                    if let Some(sender) = pending_commands.lock().await.remove(&id) {
                        let _ = sender.send(json);
                    }
                } else {
                    eprintln!("Received message without an 'id' field: {}", text);
                }
            }
            Some(Ok(_)) => {}
            Some(Err(e)) => {
                eprintln!("Error receiving message: {}", e);
                // break;
            }
            None => {
                // No message available or stream has ended
                // println!("No message available or stream has ended");
                // break;
            }
        }
    }
}