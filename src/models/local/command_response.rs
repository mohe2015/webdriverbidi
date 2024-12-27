use crate::models::local::result_data::ResultData;
use crate::models::local::{Extensible, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandResponse {
    #[serde(rename = "type")]
    response_type: String,
    id: JsUint,
    result: ResultData,
    #[serde(flatten)]
    extensible: Extensible,
}
