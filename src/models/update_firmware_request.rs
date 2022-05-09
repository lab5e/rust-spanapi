/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.2.3 lower-elian
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateFirmwareRequest : This is the request object when updating the firmware image



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateFirmwareRequest {
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl UpdateFirmwareRequest {
    /// This is the request object when updating the firmware image
    pub fn new() -> UpdateFirmwareRequest {
        UpdateFirmwareRequest {
            collection_id: None,
            version: None,
            tags: None,
        }
    }
}


