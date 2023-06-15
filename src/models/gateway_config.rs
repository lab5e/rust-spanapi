/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.6.0 truthful-holli
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GatewayConfig {
    #[serde(rename = "ciot", skip_serializing_if = "Option::is_none")]
    pub ciot: Option<Box<crate::models::GatewayCioTConfig>>,
    #[serde(rename = "inet", skip_serializing_if = "Option::is_none")]
    pub inet: Option<Box<crate::models::GatewayInetConfig>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::GatewayCustomConfig>>,
}

impl GatewayConfig {
    pub fn new() -> GatewayConfig {
        GatewayConfig {
            ciot: None,
            inet: None,
            user: None,
        }
    }
}
