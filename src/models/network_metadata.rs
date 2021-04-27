/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.12 infinite-dana
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// NetworkMetadata : Network metadata for devices.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkMetadata {
    /// Allocated IP address.
    #[serde(rename = "allocatedIp", skip_serializing_if = "Option::is_none")]
    pub allocated_ip: Option<String>,
    #[serde(rename = "allocatedAt", skip_serializing_if = "Option::is_none")]
    pub allocated_at: Option<String>,
    /// Cell ID for device. This might not be set, depending on the provider in use.
    #[serde(rename = "cellId", skip_serializing_if = "Option::is_none")]
    pub cell_id: Option<String>,
}

impl NetworkMetadata {
    /// Network metadata for devices.
    pub fn new() -> NetworkMetadata {
        NetworkMetadata {
            allocated_ip: None,
            allocated_at: None,
            cell_id: None,
        }
    }
}
