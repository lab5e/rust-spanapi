/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.1 busy-janay
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// MessageDownstream : Downstream messages are sent from the backend to the devices.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MessageDownstream {
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "sentTime", skip_serializing_if = "Option::is_none")]
    pub sent_time: Option<String>,
    #[serde(rename = "transport", skip_serializing_if = "Option::is_none")]
    pub transport: Option<crate::models::MessageTransport>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::MessageState>,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl MessageDownstream {
    /// Downstream messages are sent from the backend to the devices.
    pub fn new() -> MessageDownstream {
        MessageDownstream {
            message_id: None,
            collection_id: None,
            device_id: None,
            gateway_id: None,
            created_time: None,
            sent_time: None,
            transport: None,
            state: None,
            payload: None,
        }
    }
}
