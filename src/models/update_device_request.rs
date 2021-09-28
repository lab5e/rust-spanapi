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
pub struct UpdateDeviceRequest {
    #[serde(rename = "existingCollectionId", skip_serializing_if = "Option::is_none")]
    pub existing_collection_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// The collection id for the device. This field is optional and can be omitted if the collection id isn't changed. When changing to a new collection you must be an owner of the other collection, ie an administrator of the team that owns the new collection.
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
    #[serde(rename = "firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<Box<crate::models::FirmwareMetadata>>,
}

impl UpdateDeviceRequest {
    pub fn new() -> UpdateDeviceRequest {
        UpdateDeviceRequest {
            existing_collection_id: None,
            device_id: None,
            collection_id: None,
            imsi: None,
            imei: None,
            tags: None,
            firmware: None,
        }
    }
}


