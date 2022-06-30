/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.2.4 curable-andres
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// Collection : This is a collection

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Collection {
    /// The ID of the collection. This is assigned by the backend.
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// The team ID that owns the collection. This field is required. When you create new collections the default is to use your private team ID.
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<Box<crate::models::CollectionFirmware>>,
    /// Tags for the collection. Tags are metadata fields that you can assign to the collection.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl Collection {
    /// This is a collection
    pub fn new() -> Collection {
        Collection {
            collection_id: None,
            team_id: None,
            firmware: None,
            tags: None,
        }
    }
}
