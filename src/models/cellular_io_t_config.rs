/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.3 pitch-dark-elza
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CellularIoTConfig : This is the cellular IOT config

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CellularIoTConfig {
    #[serde(rename = "imsi", skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
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
