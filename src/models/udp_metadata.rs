/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.14 oversensitive-deante
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UdpMetadata {
    #[serde(rename = "localPort", skip_serializing_if = "Option::is_none")]
    pub local_port: Option<i32>,
    #[serde(rename = "remotePort", skip_serializing_if = "Option::is_none")]
    pub remote_port: Option<i32>,
}

impl UdpMetadata {
    pub fn new() -> UdpMetadata {
        UdpMetadata {
            local_port: None,
            remote_port: None,
        }
    }
}
