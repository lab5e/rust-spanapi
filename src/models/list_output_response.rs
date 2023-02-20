/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.0 lean-joline
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListOutputResponse : List outputs



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListOutputResponse {
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<crate::models::Output>>,
}

impl ListOutputResponse {
    /// List outputs
    pub fn new() -> ListOutputResponse {
        ListOutputResponse {
            collection_id: None,
            outputs: None,
        }
    }
}


