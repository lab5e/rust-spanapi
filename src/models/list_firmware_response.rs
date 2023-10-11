/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.7.0 actionable-aryanna
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListFirmwareResponse : List firmware response

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListFirmwareResponse {
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<crate::models::Firmware>>,
}

impl ListFirmwareResponse {
    /// List firmware response
    pub fn new() -> ListFirmwareResponse {
        ListFirmwareResponse { images: None }
    }
}
