/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.2 subversive-jamila
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// GatewayStats : This is statistics for a single gateway

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GatewayStats {
    #[serde(rename = "messagesUpstream", skip_serializing_if = "Option::is_none")]
    pub messages_upstream: Option<String>,
    #[serde(rename = "messagesDownstream", skip_serializing_if = "Option::is_none")]
    pub messages_downstream: Option<String>,
    #[serde(rename = "bytesUpstream", skip_serializing_if = "Option::is_none")]
    pub bytes_upstream: Option<String>,
    #[serde(rename = "bytesDownstream", skip_serializing_if = "Option::is_none")]
    pub bytes_downstream: Option<String>,
}

impl GatewayStats {
    /// This is statistics for a single gateway
    pub fn new() -> GatewayStats {
        GatewayStats {
            messages_upstream: None,
            messages_downstream: None,
            bytes_upstream: None,
            bytes_downstream: None,
        }
    }
}
