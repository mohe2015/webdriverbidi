use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum WebExtensionCommand {
    Install(Install),
    Uninstall(Uninstall),
}

pub type Extension = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Install {
    pub method: String,
    pub params: InstallParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallParameters {
    #[serde(rename = "extensionData")]
    pub extension_data: ExtensionData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtensionData {
    ExtensionArchivePath(ExtensionArchivePath),
    ExtensionBase64Encoded(ExtensionBase64Encoded),
    ExtensionPath(ExtensionPath),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionPath {
    #[serde(rename = "type")]
    pub extension_path_type: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionArchivePath {
    #[serde(rename = "type")]
    pub extension_archive_path_type: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionBase64Encoded {
    #[serde(rename = "type")]
    pub extension_base64_encoded_type: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Uninstall {
    pub method: String,
    pub params: UninstallParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UninstallParameters {
    pub extension: Extension,
}
