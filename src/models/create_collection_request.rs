/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.8.0 indecipherable-ferman
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateCollectionRequest : Request object when creating a collection. The collect ID is assigned by the service.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCollectionRequest {
    /// The team ID that owns the collection. This field is required. When you create new collections the default is to use your private team ID.
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<Box<crate::models::CollectionFirmware>>,
    /// Tags for the collection. Tags are metadata fields that you can assign to the collection.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl CreateCollectionRequest {
    /// Request object when creating a collection. The collect ID is assigned by the service.
    pub fn new() -> CreateCollectionRequest {
        CreateCollectionRequest {
            team_id: None,
            firmware: None,
            tags: None,
        }
    }
}
