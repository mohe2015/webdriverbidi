use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Missing command ID field")]
    MissingCommandId,

    #[error("WebSocket send error: {0}")]
    WebSocketSendError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Missing result field")]
    MissingResult,
    
    #[error("Oneshot receiver error: {0}")]
    OneshotReceiverError(#[from] tokio::sync::oneshot::error::RecvError),
}