/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.9.5 spattered-kelvin
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// Gateway : A gateway is a connection between devices and Span

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Gateway {
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "builtIn", skip_serializing_if = "Option::is_none")]
    pub built_in: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::GatewayType>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::GatewayConfig>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::GatewayStatus>,
}

impl Gateway {
    /// A gateway is a connection between devices and Span
    pub fn new() -> Gateway {
        Gateway {
            gateway_id: None,
            collection_id: None,
            name: None,
            built_in: None,
            r#type: None,
            config: None,
            tags: None,
            status: None,
        }
    }
}
