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

impl PartionKey {
    pub fn new(user_context: Option<String>, source_origin: Option<String>) -> Self {
        Self {
            user_context,
            source_origin,
            extensible: Extensible::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCookies {
    pub method: String,
    pub params: GetCookiesParameters,
}

impl GetCookies {
    pub fn new(params: GetCookiesParameters) -> Self {
        Self {
            method: "storage.getCookies".to_string(),
            params,
        }
    }
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

impl CookieFilter {
    pub fn new(
        name: Option<String>,
        value: Option<BytesValue>,
        domain: Option<String>,
        path: Option<String>,
        size: Option<JsUint>,
        http_only: Option<bool>,
        secure: Option<bool>,
        same_site: Option<SameSite>,
        expiry: Option<JsUint>,
        extensible: Extensible,
    ) -> Self {
        Self {
            name,
            value,
            domain,
            path,
            size,
            http_only,
            secure,
            same_site,
            expiry,
            extensible,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrowsingContextPartitionDescriptor {
    #[serde(rename = "type")]
    pub browsing_context_partition_descriptor_type: String,
    pub context: browsing_context::BrowsingContext,
}

impl BrowsingContextPartitionDescriptor {
    pub fn new(context: browsing_context::BrowsingContext) -> Self {
        Self {
            browsing_context_partition_descriptor_type: "context".to_string(),
            context,
        }
    }
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

impl StorageKeyPartitionDescriptor {
    pub fn new(user_context: Option<String>, source_origin: Option<String>) -> Self {
        Self {
            storage_key_partition_descriptor_type: "storageKey".to_string(),
            user_context,
            source_origin,
            extensible: Extensible::new(),
        }
    }
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

impl GetCookiesParameters {
    pub fn new(filter: Option<CookieFilter>, partition: Option<PartitionDescriptor>) -> Self {
        Self { filter, partition }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookie {
    pub method: String,
    pub params: SetCookieParameters,
}

impl SetCookie {
    pub fn new(params: SetCookieParameters) -> Self {
        Self {
            method: "storage.setCookie".to_string(),
            params,
        }
    }
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

impl PartialCookie {
    pub fn new(
        name: String,
        value: BytesValue,
        domain: String,
        path: Option<String>,
        http_only: Option<bool>,
        secure: Option<bool>,
        same_site: Option<SameSite>,
        expiry: Option<JsUint>,
        extensible: Extensible,
    ) -> Self {
        Self {
            name,
            value,
            domain,
            path,
            http_only,
            secure,
            same_site,
            expiry,
            extensible,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCookieParameters {
    pub cookie: PartialCookie,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<PartitionDescriptor>,
}

impl SetCookieParameters {
    pub fn new(cookie: PartialCookie, partition: Option<PartitionDescriptor>) -> Self {
        Self { cookie, partition }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookies {
    pub method: String,
    pub params: DeleteCookiesParameters,
}

impl DeleteCookies {
    pub fn new(params: DeleteCookiesParameters) -> Self {
        Self {
            method: "storage.deleteCookies".to_string(),
            params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCookiesParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CookieFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<PartitionDescriptor>,
}

impl DeleteCookiesParameters {
    pub fn new(filter: Option<CookieFilter>, partition: Option<PartitionDescriptor>) -> Self {
        Self { filter, partition }
    }
}
