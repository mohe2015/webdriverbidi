use crate::models::local::{ErrorCode, Extensible, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    response_type: String,
    id: Option<JsUint>,
    error: ErrorCode,
    message: String,
    stacktrace: Option<String>,
    #[serde(flatten)]
    extensible: Extensible,
}
