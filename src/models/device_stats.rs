/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.6.0 truthful-holli
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// DeviceStats : This is the statistics for a single device

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeviceStats {
    #[serde(rename = "bytesUpstreamMb", skip_serializing_if = "Option::is_none")]
    pub bytes_upstream_mb: Option<f32>,
    #[serde(rename = "bytesDownstreamMb", skip_serializing_if = "Option::is_none")]
    pub bytes_downstream_mb: Option<f32>,
    #[serde(rename = "messagesUpstream", skip_serializing_if = "Option::is_none")]
    pub messages_upstream: Option<i32>,
    #[serde(rename = "messagesDownstream", skip_serializing_if = "Option::is_none")]
    pub messages_downstream: Option<i32>,
    #[serde(rename = "sessionCount", skip_serializing_if = "Option::is_none")]
    pub session_count: Option<i32>,
}

impl DeviceStats {
    /// This is the statistics for a single device
    pub fn new() -> DeviceStats {
        DeviceStats {
            bytes_upstream_mb: None,
            bytes_downstream_mb: None,
            messages_upstream: None,
            messages_downstream: None,
            session_count: None,
        }
    }
}
