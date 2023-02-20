/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.0 lean-joline
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// OutputConfig : Configuration for outputs.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OutputConfig {
    /// URL for the webhook.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "basicAuthUser", skip_serializing_if = "Option::is_none")]
    pub basic_auth_user: Option<String>,
    #[serde(rename = "basicAuthPass", skip_serializing_if = "Option::is_none")]
    pub basic_auth_pass: Option<String>,
    #[serde(rename = "customHeaderName", skip_serializing_if = "Option::is_none")]
    pub custom_header_name: Option<String>,
    #[serde(rename = "customHeaderValue", skip_serializing_if = "Option::is_none")]
    pub custom_header_value: Option<String>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "eventName", skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(rename = "asIsPayload", skip_serializing_if = "Option::is_none")]
    pub as_is_payload: Option<bool>,
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// MQTT configuration: Disable certificate checks. Default is off.
    #[serde(rename = "disableCertCheck", skip_serializing_if = "Option::is_none")]
    pub disable_cert_check: Option<bool>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "topicName", skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    #[serde(rename = "topicTemplate", skip_serializing_if = "Option::is_none")]
    pub topic_template: Option<String>,
    #[serde(rename = "payloadFormat", skip_serializing_if = "Option::is_none")]
    pub payload_format: Option<String>,
    #[serde(rename = "payloadTemplate", skip_serializing_if = "Option::is_none")]
    pub payload_template: Option<String>,
}

impl OutputConfig {
    /// Configuration for outputs.
    pub fn new() -> OutputConfig {
        OutputConfig {
            url: None,
            basic_auth_user: None,
            basic_auth_pass: None,
            custom_header_name: None,
            custom_header_value: None,
            host: None,
            port: None,
            key: None,
            event_name: None,
            as_is_payload: None,
            endpoint: None,
            disable_cert_check: None,
            username: None,
            password: None,
            client_id: None,
            topic_name: None,
            topic_template: None,
            payload_format: None,
            payload_template: None,
        }
    }
}
