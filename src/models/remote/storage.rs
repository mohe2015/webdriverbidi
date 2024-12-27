use crate::models::remote::browsing_context;
use crate::models::remote::network::{BytesValue, SameSite};
use crate::models::remote::{Extensible, JsUint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StorageCommand {
    DeleteCookies(DeleteCookies),
    GetCookies(GetCookies),
    SetCookie(SetCookie),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartionKey {
    #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
    user_context: Option<String>,
    #[serde(rename = "sourceOrigin", skip_serializing_if = "Option::is_none")]
    source_origin: Option<String>,
    extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCookies {
    method: String,
    params: GetCookiesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookieFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<BytesValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<JsUint>,
    #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
    http_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure: Option<bool>,
    #[serde(rename = "sameSite", skip_serializing_if = "Option::is_none")]
    same_site: Option<SameSite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiry: Option<JsUint>,
    extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowsingContextPartitionDescriptor {
    #[serde(rename = "type")]
    browsing_context_partition_descriptor_type: String,
    context: browsing_context::BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageKeyPartitionDescriptor {
    #[serde(rename = "type")]
    storage_key_partition_descriptor_type: String,
    #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
    user_context: Option<String>,
    #[serde(rename = "sourceOrigin", skip_serializing_if = "Option::is_none")]
    source_origin: Option<String>,
    extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PartitionDescriptor {
    BrowsingContextPartitionDescriptor(BrowsingContextPartitionDescriptor),
    StorageKeyPartitionDescriptor(StorageKeyPartitionDescriptor),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCookiesParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<CookieFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookie {
    method: String,
    params: SetCookieParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialCookie {
    name: String,
    value: BytesValue,
    domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
    http_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secure: Option<bool>,
    #[serde(rename = "sameSite", skip_serializing_if = "Option::is_none")]
    same_site: Option<SameSite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiry: Option<JsUint>,
    extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookieParameters {
    cookie: PartialCookie,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookies {
    method: String,
    params: DeleteCookiesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookiesParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<CookieFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition: Option<PartitionDescriptor>,
}
