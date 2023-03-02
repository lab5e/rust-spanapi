/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.2 nonviolent-adelbert
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// InetMetadata : Metadata for devices connected via the internet gateway. This metadata shows the configuration for the last message transmission.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InetMetadata {
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    #[serde(rename = "remoteAddress", skip_serializing_if = "Option::is_none")]
    pub remote_address: Option<String>,
    #[serde(rename = "certificateSerial", skip_serializing_if = "Option::is_none")]
    pub certificate_serial: Option<String>,
}

impl InetMetadata {
    /// Metadata for devices connected via the internet gateway. This metadata shows the configuration for the last message transmission.
    pub fn new() -> InetMetadata {
        InetMetadata {
            gateway_id: None,
            last_update: None,
            remote_address: None,
            certificate_serial: None,
        }
    }
}


