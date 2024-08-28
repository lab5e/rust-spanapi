/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.1 humorous-jaron
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// Device : This a device

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Device {
    /// The device ID is assigned by the backend.
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// Tags are metadata for the device that you can set. These are just strings.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<Box<crate::models::FirmwareMetadata>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::DeviceConfig>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::DeviceMetadata>>,
    #[serde(rename = "lastGatewayId", skip_serializing_if = "Option::is_none")]
    pub last_gateway_id: Option<String>,
    #[serde(rename = "lastTransport", skip_serializing_if = "Option::is_none")]
    pub last_transport: Option<crate::models::MessageTransport>,
    #[serde(rename = "lastReceived", skip_serializing_if = "Option::is_none")]
    pub last_received: Option<String>,
    #[serde(rename = "lastPayload", skip_serializing_if = "Option::is_none")]
    pub last_payload: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Device {
    /// This a device
    pub fn new() -> Device {
        Device {
            device_id: None,
            collection_id: None,
            tags: None,
            firmware: None,
            config: None,
            metadata: None,
            last_gateway_id: None,
            last_transport: None,
            last_received: None,
            last_payload: None,
            enabled: None,
        }
    }
}
