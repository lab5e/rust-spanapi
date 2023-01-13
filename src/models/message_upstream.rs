/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.3.0 grouchy-aloysius
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// MessageUpstream : This is the messages sent from the device to the backend service



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MessageUpstream {
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "transport", skip_serializing_if = "Option::is_none")]
    pub transport: Option<crate::models::MessageTransport>,
    #[serde(rename = "received", skip_serializing_if = "Option::is_none")]
    pub received: Option<String>,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl MessageUpstream {
    /// This is the messages sent from the device to the backend service
    pub fn new() -> MessageUpstream {
        MessageUpstream {
            message_id: None,
            collection_id: None,
            device_id: None,
            gateway_id: None,
            transport: None,
            received: None,
            payload: None,
        }
    }
}


