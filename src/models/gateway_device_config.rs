/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.9.5 spattered-kelvin
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// GatewayDeviceConfig : Configuration parameters for a device in a user-managed gateway. The configuration parameters depends on the type of gateway.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GatewayDeviceConfig {
    /// This is the ID of the gateway this configuration applies to.
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<::std::collections::HashMap<String, String>>,
}

impl GatewayDeviceConfig {
    /// Configuration parameters for a device in a user-managed gateway. The configuration parameters depends on the type of gateway.
    pub fn new() -> GatewayDeviceConfig {
        GatewayDeviceConfig {
            gateway_id: None,
            params: None,
        }
    }
}
