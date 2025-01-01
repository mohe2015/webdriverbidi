use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

// --------------------------------------------------

use futures::stream::Stream;
use futures::task::{Context, Poll};
use log::{debug, error};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

// --------------------------------------------------

const ID_FIELD: &str = "id";

// --------------------------------------------------

/// Handles incoming WebSocket messages.
///
/// # Arguments
///
/// * `websocket_stream` - An `Arc` wrapped `Mutex` protecting a `WebSocketStream`
/// that can be either a plain TCP stream or a TLS-encrypted stream. This stream
/// is used to receive WebSocket messages.
/// * `pending_commands` - An `Arc` wrapped `Mutex` protecting a `HashMap` where
/// the keys are command IDs (u64) and the values are `oneshot::Sender<Value>`
/// channels. These channels are used to send responses back to the pending commands.
pub async fn handle_messages(
    websocket_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    pending_commands: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>>,
) {
    loop {
        let message = {
            debug!("Locking the WebSocket stream mutex");
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

        debug!("Received message: {:?}", message);

        match message {
            Some(Ok(Message::Text(text))) => match serde_json::from_str::<Value>(&text) {
                Ok(json) => {
                    if let Some(id) = json.get(ID_FIELD).and_then(|id| id.as_u64()) {
                        if let Some(sender) = pending_commands.lock().await.remove(&id) {
                            debug!("Sending JSON to receiver: {:?}", json);
                            let _ = sender.send(json);
                        }
                    } else {
                        error!("Received message without an 'id' field: {}", text);
                    }
                }
                Err(e) => {
                    error!("Failed to parse JSON: {:?}", e);
                }
            },
            Some(Ok(_)) => {}
            Some(Err(e)) => {
                error!("Error receiving message: {}", e);
            }
            None => {}
        }
    }
}
