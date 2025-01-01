use crate::local::result_data::ResultData;
use crate::local::{Extensible, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandResponse {
    #[serde(rename = "type")]
    pub response_type: String,
    pub id: JsUint,
    pub result: ResultData,
    #[serde(flatten)]
    pub extensible: Extensible,
}
