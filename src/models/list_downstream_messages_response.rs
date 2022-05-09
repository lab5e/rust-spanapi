/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.2.3 lower-elian
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListDownstreamMessagesResponse : Response object when listing downstream messages



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListDownstreamMessagesResponse {
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::models::MessageDownstream>>,
}

impl ListDownstreamMessagesResponse {
    /// Response object when listing downstream messages
    pub fn new() -> ListDownstreamMessagesResponse {
        ListDownstreamMessagesResponse {
            messages: None,
        }
    }
}


