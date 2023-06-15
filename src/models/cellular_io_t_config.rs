/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.6.0 truthful-holli
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CellularIoTConfig : This is the cellular IOT config

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CellularIoTConfig {
    #[serde(rename = "imsi", skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    /// on your device. This is the primary identifier for your device on the network.  The IMEI number is the unique ID for your hardware as
    #[serde(rename = "imei", skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
}

impl CellularIoTConfig {
    /// This is the cellular IOT config
    pub fn new() -> CellularIoTConfig {
        CellularIoTConfig {
            imsi: None,
            imei: None,
        }
    }
}
