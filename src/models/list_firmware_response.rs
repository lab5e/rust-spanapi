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
pub struct ListFirmwareResponse {
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::Firmware>>,
}

impl ListFirmwareResponse {
    pub fn new() -> ListFirmwareResponse {
        ListFirmwareResponse { images: None }
    }
}
