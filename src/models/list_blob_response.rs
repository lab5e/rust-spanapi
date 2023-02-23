/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.1 busy-janay
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListBlobResponse : Response object when listing blobs for a collection

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListBlobResponse {
    #[serde(rename = "blobs", skip_serializing_if = "Option::is_none")]
    pub blobs: Option<Vec<crate::models::Blob>>,
}

impl ListBlobResponse {
    /// Response object when listing blobs for a collection
    pub fn new() -> ListBlobResponse {
        ListBlobResponse { blobs: None }
    }
}