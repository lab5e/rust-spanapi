/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.8.0 indecipherable-ferman
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// DeviceConfig : This is the configuration for the device via the various gateways.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeviceConfig {
    #[serde(rename = "ciot", skip_serializing_if = "Option::is_none")]
    pub ciot: Option<Box<crate::models::CellularIoTConfig>>,
    /// This is the configuration for an internet-connected device. There are currently no configuration options for internet devices; the device is identified via the clientcertificate.  This is empty since there's no configuration required for the internet  gateway
    #[serde(rename = "inet", skip_serializing_if = "Option::is_none")]
    pub inet: Option<serde_json::Value>,
    #[serde(rename = "gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<::std::collections::HashMap<String, crate::models::GatewayDeviceConfig>>,
}

impl DeviceConfig {
    /// This is the configuration for the device via the various gateways.
    pub fn new() -> DeviceConfig {
        DeviceConfig {
            ciot: None,
            inet: None,
            gateway: None,
        }
    }
}
