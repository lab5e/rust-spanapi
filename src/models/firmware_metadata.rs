/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.7 only-deshaun
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// FirmwareMetadata : Metadata about firmware on devices.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirmwareMetadata {
    #[serde(rename = "currentFirmwareId", skip_serializing_if = "Option::is_none")]
    pub current_firmware_id: Option<String>,
    #[serde(rename = "targetFirmwareId", skip_serializing_if = "Option::is_none")]
    pub target_firmware_id: Option<String>,
    /// Last reported firmware version.
    #[serde(rename = "firmwareVersion", skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "modelNumber", skip_serializing_if = "Option::is_none")]
    pub model_number: Option<String>,
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// State of the firmware.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "stateMessage", skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
}

impl FirmwareMetadata {
    /// Metadata about firmware on devices.
    pub fn new() -> FirmwareMetadata {
        FirmwareMetadata {
            current_firmware_id: None,
            target_firmware_id: None,
            firmware_version: None,
            serial_number: None,
            model_number: None,
            manufacturer: None,
            state: None,
            state_message: None,
        }
    }
}

