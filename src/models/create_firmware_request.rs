/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.3.0 grouchy-aloysius
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateFirmwareRequest : Create a new firmware image

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFirmwareRequest {
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl CreateFirmwareRequest {
    /// Create a new firmware image
    pub fn new() -> CreateFirmwareRequest {
        CreateFirmwareRequest {
            image: None,
            version: None,
            filename: None,
            tags: None,
        }
    }
}
