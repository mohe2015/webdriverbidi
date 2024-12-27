use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum WebExtensionCommand {
    Install(Install),
    Uninstall(Uninstall),
}

pub type Extension = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Install {
    method: String,
    params: InstallParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallParameters {
    #[serde(rename = "extensionData")]
    extension_data: ExtensionData,
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
    extension_path_type: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionArchivePath {
    #[serde(rename = "type")]
    extension_archive_path_type: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionBase64Encoded {
    #[serde(rename = "type")]
    extension_base64_encoded_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Uninstall {
    method: String,
    params: UninstallParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UninstallParameters {
    extension: Extension,
}
