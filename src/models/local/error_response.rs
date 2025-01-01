use crate::local::{ErrorCode, Extensible, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub id: Option<JsUint>,
    pub error: ErrorCode,
    pub message: String,
    pub stacktrace: Option<String>,
    #[serde(flatten)]
    pub extensible: Extensible,
}
