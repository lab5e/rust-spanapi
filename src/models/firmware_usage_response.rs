/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.10 hulking-betty
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirmwareUsageResponse {
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "targeted", skip_serializing_if = "Option::is_none")]
    pub targeted: Option<Vec<String>>,
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<Vec<String>>,
}

impl FirmwareUsageResponse {
    pub fn new() -> FirmwareUsageResponse {
        FirmwareUsageResponse {
            image_id: None,
            targeted: None,
            current: None,
        }
    }
}
