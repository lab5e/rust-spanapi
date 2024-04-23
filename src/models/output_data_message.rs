/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.9.5 spattered-kelvin
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// OutputDataMessage : The output data message contains payload plus metadata for a payload received from a device.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OutputDataMessage {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::OutputMessageType>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::Device>>,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    /// Received time for message. Value is ms since epoch.
    #[serde(rename = "received", skip_serializing_if = "Option::is_none")]
    pub received: Option<String>,
    #[serde(rename = "transport", skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
    #[serde(rename = "udpMetaData", skip_serializing_if = "Option::is_none")]
    pub udp_meta_data: Option<Box<crate::models::UdpMetadata>>,
    #[serde(rename = "coapMetaData", skip_serializing_if = "Option::is_none")]
    pub coap_meta_data: Option<Box<crate::models::CoApMetadata>>,
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "mqttMetaData", skip_serializing_if = "Option::is_none")]
    pub mqtt_meta_data: Option<Box<crate::models::MqttMetadata>>,
    #[serde(rename = "gatewayMetaData", skip_serializing_if = "Option::is_none")]
    pub gateway_meta_data: Option<Box<crate::models::GatewayMetadata>>,
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
}

impl OutputDataMessage {
    /// The output data message contains payload plus metadata for a payload received from a device.
    pub fn new() -> OutputDataMessage {
        OutputDataMessage {
            r#type: None,
            device: None,
            payload: None,
            received: None,
            transport: None,
            udp_meta_data: None,
            coap_meta_data: None,
            message_id: None,
            mqtt_meta_data: None,
            gateway_meta_data: None,
            gateway_id: None,
        }
    }
}
