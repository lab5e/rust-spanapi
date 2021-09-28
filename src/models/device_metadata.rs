/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.17 enhanced-allie
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceMetadata {
    #[serde(rename = "simOperator", skip_serializing_if = "Option::is_none")]
    pub sim_operator: Option<Box<crate::models::NetworkOperator>>,
}

impl DeviceMetadata {
    pub fn new() -> DeviceMetadata {
        DeviceMetadata { sim_operator: None }
    }
}
