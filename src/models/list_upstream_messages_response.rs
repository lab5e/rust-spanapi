/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.3.0 grouchy-aloysius
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListUpstreamMessagesResponse : Response object when listing upstream messages

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListUpstreamMessagesResponse {
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::models::MessageUpstream>>,
}

impl ListUpstreamMessagesResponse {
    /// Response object when listing upstream messages
    pub fn new() -> ListUpstreamMessagesResponse {
        ListUpstreamMessagesResponse { messages: None }
    }
}
