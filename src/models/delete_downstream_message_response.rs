/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.6.0 truthful-holli
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// DeleteDownstreamMessageResponse : Response object when deleting a downstream message

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteDownstreamMessageResponse {
    #[serde(rename = "messageId", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

impl DeleteDownstreamMessageResponse {
    /// Response object when deleting a downstream message
    pub fn new() -> DeleteDownstreamMessageResponse {
        DeleteDownstreamMessageResponse { message_id: None }
    }
}
