/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.1 humorous-jaron
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// MqttMetadata : MQTT metadata for messages received through one of the MQTT endpoints. This is an EXPERIMENTAL feature.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MqttMetadata {
    #[serde(rename = "topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

impl MqttMetadata {
    /// MQTT metadata for messages received through one of the MQTT endpoints. This is an EXPERIMENTAL feature.
    pub fn new() -> MqttMetadata {
        MqttMetadata { topic: None }
    }
}
