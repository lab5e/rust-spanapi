/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.3 pitch-dark-elza
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// UdpMetadata : UDP metadata for messages receveied through one of the UDP endpoints

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UdpMetadata {
    #[serde(rename = "localPort", skip_serializing_if = "Option::is_none")]
    pub local_port: Option<i32>,
    #[serde(rename = "remotePort", skip_serializing_if = "Option::is_none")]
    pub remote_port: Option<i32>,
}

impl UdpMetadata {
    /// UDP metadata for messages receveied through one of the UDP endpoints
    pub fn new() -> UdpMetadata {
        UdpMetadata {
            local_port: None,
            remote_port: None,
        }
    }
}
