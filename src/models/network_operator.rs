/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.18 disgruntled-jerald
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// NetworkOperator : Operator holds information on the network operator. There might be several operators involved; one operator is running the network your devices are connected to and the SIM card in your device belongs to a different operator.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkOperator {
    /// The Mobil Country Code for the operator.
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
}

impl NetworkOperator {
    /// Operator holds information on the network operator. There might be several operators involved; one operator is running the network your devices are connected to and the SIM card in your device belongs to a different operator.
    pub fn new() -> NetworkOperator {
        NetworkOperator {
            mcc: None,
            mnc: None,
            country: None,
            network: None,
            country_code: None,
        }
    }
}
