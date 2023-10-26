/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.8.0 indecipherable-ferman
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// BlobStats : Statistics for a single blob

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlobStats {
    #[serde(rename = "blobBytesMb", skip_serializing_if = "Option::is_none")]
    pub blob_bytes_mb: Option<f32>,
}

impl BlobStats {
    /// Statistics for a single blob
    pub fn new() -> BlobStats {
        BlobStats {
            blob_bytes_mb: None,
        }
    }
}