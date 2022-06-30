/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.2.4 curable-andres
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateOutputRequest : Request type to update outputs



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateOutputRequest {
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::OutputType>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::OutputConfig>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl UpdateOutputRequest {
    /// Request type to update outputs
    pub fn new() -> UpdateOutputRequest {
        UpdateOutputRequest {
            collection_id: None,
            _type: None,
            config: None,
            enabled: None,
            tags: None,
        }
    }
}


