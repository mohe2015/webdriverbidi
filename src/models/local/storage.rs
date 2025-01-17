use crate::local::{network, Extensible};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StorageResult {
    DeleteCookiesResult(DeleteCookiesResult),
    GetCookiesResult(GetCookiesResult),
    SetCookieResult(SetCookieResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PartitionKey {
    #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
    pub user_context: Option<String>,
    #[serde(rename = "sourceOrigin", skip_serializing_if = "Option::is_none")]
    pub source_origin: Option<String>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCookiesResult {
    pub cookies: Vec<network::Cookie>,
    #[serde(rename = "partitionKey")]
    pub partition_key: PartitionKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetCookieResult {
    #[serde(rename = "partitionKey")]
    pub partition_key: PartitionKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteCookiesResult {
    #[serde(rename = "partitionKey")]
    pub partition_key: PartitionKey,
}
