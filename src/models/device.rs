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
pub struct Device {
    /// The device ID is assigned by the backend.
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// The IMSI is the unique ID for the (e|nu|whatever)SIM card on your device. This is the primary identifier for your device on the network.
    #[serde(rename = "imsi", skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    /// The IMEI number is the unique ID for your hardware as seen by the network. Obviously you might have a completely different view on things.
    #[serde(rename = "imei", skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    /// Tags are metadata for the device that you can set. These are just strings.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<Box<crate::models::NetworkMetadata>>,
    #[serde(rename = "firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<Box<crate::models::FirmwareMetadata>>,
}

impl Device {
    pub fn new() -> Device {
        Device {
            device_id: None,
            collection_id: None,
            imsi: None,
            imei: None,
            tags: None,
            network: None,
            firmware: None,
        }
    }
}


