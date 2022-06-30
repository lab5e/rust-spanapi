/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.2.4 curable-andres
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// AddDownstreamMessageRequest : This is the request object to send messages out to the devices



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddDownstreamMessageRequest {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl AddDownstreamMessageRequest {
    /// This is the request object to send messages out to the devices
    pub fn new() -> AddDownstreamMessageRequest {
        AddDownstreamMessageRequest {
            payload: None,
        }
    }
}


