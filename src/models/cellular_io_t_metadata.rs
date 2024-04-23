/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.9.5 spattered-kelvin
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CellularIoTMetadata : This is the metadata for a Cellular IoT device connected via an APN.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CellularIoTMetadata {
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    /// Allocated IP address.
    #[serde(rename = "allocatedIp", skip_serializing_if = "Option::is_none")]
    pub allocated_ip: Option<String>,
    #[serde(rename = "allocatedAt", skip_serializing_if = "Option::is_none")]
    pub allocated_at: Option<String>,
    #[serde(rename = "cellId", skip_serializing_if = "Option::is_none")]
    pub cell_id: Option<String>,
    /// the provider in use.  The Mobile Country Code for the operator.
    #[serde(rename = "mcc", skip_serializing_if = "Option::is_none")]
    pub mcc: Option<i32>,
    #[serde(rename = "mnc", skip_serializing_if = "Option::is_none")]
    pub mnc: Option<i32>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    #[serde(rename = "lastImsi", skip_serializing_if = "Option::is_none")]
    pub last_imsi: Option<String>,
    #[serde(rename = "lastImei", skip_serializing_if = "Option::is_none")]
    pub last_imei: Option<String>,
}

impl CellularIoTMetadata {
    /// This is the metadata for a Cellular IoT device connected via an APN.
    pub fn new() -> CellularIoTMetadata {
        CellularIoTMetadata {
            gateway_id: None,
            allocated_ip: None,
            allocated_at: None,
            cell_id: None,
            mcc: None,
            mnc: None,
            country: None,
            network: None,
            country_code: None,
            last_update: None,
            last_imsi: None,
            last_imei: None,
        }
    }
}
