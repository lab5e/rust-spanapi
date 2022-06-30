/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.2.4 curable-andres
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateDeviceRequest : Updating the device

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateDeviceRequest {
    /// The collection id for the device. This field is optional and can be omitted if the collection id isn't changed. When changing to a new collection you must be an owner of the other collection, ie an administrator of the team that owns the new collection.
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// Deprecated: this is replaced by the Config type The IMSI is the unique ID for the (e|nu|whatever)SIM card on your device. This is the primary identifier for your device on the network.
    #[serde(rename = "imsi", skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    /// Deprecated: This is replaced by the Config type The IMEI number is the unique ID for your hardware as seen by the network. Obviously you might have a completely different view on things.
    #[serde(rename = "imei", skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    /// Tags are metadata for the device that you can set. These are just strings.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<Box<crate::models::FirmwareMetadata>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::DeviceConfig>>,
}

impl UpdateDeviceRequest {
    /// Updating the device
    pub fn new() -> UpdateDeviceRequest {
        UpdateDeviceRequest {
            collection_id: None,
            imsi: None,
            imei: None,
            tags: None,
            firmware: None,
            config: None,
        }
    }
}
