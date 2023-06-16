/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.6.1 squirming-codi
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// FirmwareStats : Statistics for a single firmware image

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FirmwareStats {
    #[serde(rename = "firmwareImageSize", skip_serializing_if = "Option::is_none")]
    pub firmware_image_size: Option<i32>,
}

impl FirmwareStats {
    /// Statistics for a single firmware image
    pub fn new() -> FirmwareStats {
        FirmwareStats {
            firmware_image_size: None,
        }
    }
}
