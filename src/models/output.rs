/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.17 enhanced-allie
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// Output : Output resource. The configuration depends on the kind of output type. There are five outputs: Webhooks, UDP forwarding, IFTTT events, MQTT client and MQTT broker. The MQTT broker output is just used to configure the built-in MQTT broker in Span.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
    #[serde(rename = "outputId", skip_serializing_if = "Option::is_none")]
    pub output_id: Option<String>,
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::OutputType>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::OutputConfig>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl Output {
    /// Output resource. The configuration depends on the kind of output type. There are five outputs: Webhooks, UDP forwarding, IFTTT events, MQTT client and MQTT broker. The MQTT broker output is just used to configure the built-in MQTT broker in Span.
    pub fn new() -> Output {
        Output {
            output_id: None,
            collection_id: None,
            _type: None,
            config: None,
            enabled: None,
            tags: None,
        }
    }
}
