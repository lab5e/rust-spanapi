/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.2 subversive-jamila
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateGatewayBody {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::GatewayType>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::GatewayConfig>>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl CreateGatewayBody {
    pub fn new() -> CreateGatewayBody {
        CreateGatewayBody {
            name: None,
            r#type: None,
            config: None,
            tags: None,
        }
    }
}
