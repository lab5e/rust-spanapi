/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.9.6 authoritarian-betty
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// GatewayMetadata : Metadata for gateway transports. The actual contents varies from gateway to gateway

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GatewayMetadata {
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
}

impl GatewayMetadata {
    /// Metadata for gateway transports. The actual contents varies from gateway to gateway
    pub fn new() -> GatewayMetadata {
        GatewayMetadata { metadata: None }
    }
}
