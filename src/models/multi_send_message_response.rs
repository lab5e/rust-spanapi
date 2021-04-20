/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.7 prized-adelbert
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// MultiSendMessageResponse : Broadcast message result. The errors array contains the list of errors ocurred when sending a message.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiSendMessageResponse {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::MessageSendResult>>,
    #[serde(rename = "sent", skip_serializing_if = "Option::is_none")]
    pub sent: Option<i32>,
    #[serde(rename = "failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
}

impl MultiSendMessageResponse {
    /// Broadcast message result. The errors array contains the list of errors ocurred when sending a message.
    pub fn new() -> MultiSendMessageResponse {
        MultiSendMessageResponse {
            errors: None,
            sent: None,
            failed: None,
        }
    }
}


