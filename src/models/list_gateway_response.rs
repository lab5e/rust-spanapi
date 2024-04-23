/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.9.5 spattered-kelvin
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListGatewayResponse : Response when listing gateways

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListGatewayResponse {
    #[serde(rename = "gateways", skip_serializing_if = "Option::is_none")]
    pub gateways: Option<Vec<crate::models::Gateway>>,
}

impl ListGatewayResponse {
    /// Response when listing gateways
    pub fn new() -> ListGatewayResponse {
        ListGatewayResponse { gateways: None }
    }
}
