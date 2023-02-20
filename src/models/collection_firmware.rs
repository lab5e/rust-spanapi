/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.0 lean-joline
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CollectionFirmware : This is the firmware configuration for a collection.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CollectionFirmware {
    /// The current firmware is the firmware that the devices are currently using.
    #[serde(rename = "currentFirmwareId", skip_serializing_if = "Option::is_none")]
    pub current_firmware_id: Option<String>,
    /// The target firmware is set to the desired firmware image for the devices in this collection. If the management is set to \"device\" this will only be used if the target firmware isn't set on the device itself.
    #[serde(rename = "targetFirmwareId", skip_serializing_if = "Option::is_none")]
    pub target_firmware_id: Option<String>,
    #[serde(rename = "management", skip_serializing_if = "Option::is_none")]
    pub management: Option<crate::models::FirmwareManagement>,
}

impl CollectionFirmware {
    /// This is the firmware configuration for a collection.
    pub fn new() -> CollectionFirmware {
        CollectionFirmware {
            current_firmware_id: None,
            target_firmware_id: None,
            management: None,
        }
    }
}


