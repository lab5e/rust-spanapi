/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.8.0 indecipherable-ferman
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// GatewayStats : This is statistics for a single gateway

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GatewayStats {
    #[serde(rename = "messagesUpstream", skip_serializing_if = "Option::is_none")]
    pub messages_upstream: Option<i32>,
    #[serde(rename = "messagesDownstream", skip_serializing_if = "Option::is_none")]
    pub messages_downstream: Option<i32>,
    #[serde(rename = "bytesUpstreamMb", skip_serializing_if = "Option::is_none")]
    pub bytes_upstream_mb: Option<f32>,
    #[serde(rename = "bytesDownstreamMb", skip_serializing_if = "Option::is_none")]
    pub bytes_downstream_mb: Option<f32>,
}

impl GatewayStats {
    /// This is statistics for a single gateway
    pub fn new() -> GatewayStats {
        GatewayStats {
            messages_upstream: None,
            messages_downstream: None,
            bytes_upstream_mb: None,
            bytes_downstream_mb: None,
        }
    }
}
