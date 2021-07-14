/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.16 spooky-devante
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListCollectionResponse : Collection list. The list contains all the collections you have access to.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListCollectionResponse {
    #[serde(rename = "collections", skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<crate::models::Collection>>,
}

impl ListCollectionResponse {
    /// Collection list. The list contains all the collections you have access to.
    pub fn new() -> ListCollectionResponse {
        ListCollectionResponse {
            collections: None,
        }
    }
}


