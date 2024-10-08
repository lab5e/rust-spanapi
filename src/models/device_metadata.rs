/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.3 pitch-dark-elza
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// DeviceMetadata : This is the metadata for devices.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeviceMetadata {
    #[serde(rename = "ciot", skip_serializing_if = "Option::is_none")]
    pub ciot: Option<Box<crate::models::CellularIoTMetadata>>,
    #[serde(rename = "inet", skip_serializing_if = "Option::is_none")]
    pub inet: Option<Box<crate::models::InetMetadata>>,
    #[serde(rename = "gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Box<crate::models::GatewayDeviceMetadata>>,
}

impl DeviceMetadata {
    /// This is the metadata for devices.
    pub fn new() -> DeviceMetadata {
        DeviceMetadata {
            ciot: None,
            inet: None,
            gateway: None,
        }
    }
}
