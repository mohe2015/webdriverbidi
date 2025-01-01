use crate::remote::browsing_context;
use crate::remote::network::{BytesValue, SameSite};
use crate::remote::{Extensible, JsUint};
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
    pub user_context: Option<String>,
    #[serde(rename = "sourceOrigin", skip_serializing_if = "Option::is_none")]
    pub source_origin: Option<String>,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCookies {
    pub method: String,
    pub params: GetCookiesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookieFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<BytesValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<JsUint>,
    #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(rename = "sameSite", skip_serializing_if = "Option::is_none")]
    pub same_site: Option<SameSite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<JsUint>,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowsingContextPartitionDescriptor {
    #[serde(rename = "type")]
    pub browsing_context_partition_descriptor_type: String,
    pub context: browsing_context::BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageKeyPartitionDescriptor {
    #[serde(rename = "type")]
    pub storage_key_partition_descriptor_type: String,
    #[serde(rename = "userContext", skip_serializing_if = "Option::is_none")]
    pub user_context: Option<String>,
    #[serde(rename = "sourceOrigin", skip_serializing_if = "Option::is_none")]
    pub source_origin: Option<String>,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PartitionDescriptor {
    BrowsingContextPartitionDescriptor(BrowsingContextPartitionDescriptor),
    StorageKeyPartitionDescriptor(StorageKeyPartitionDescriptor),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCookiesParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CookieFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookie {
    pub method: String,
    pub params: SetCookieParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialCookie {
    pub name: String,
    pub value: BytesValue,
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "httpOnly", skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(rename = "sameSite", skip_serializing_if = "Option::is_none")]
    pub same_site: Option<SameSite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<JsUint>,
    pub extensible: Extensible,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookieParameters {
    pub cookie: PartialCookie,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<PartitionDescriptor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookies {
    pub method: String,
    pub params: DeleteCookiesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookiesParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CookieFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<PartitionDescriptor>,
}
