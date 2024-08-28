/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.1 humorous-jaron
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// AddDownstreamMessageBody : This is the request object to send messages out to the devices

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddDownstreamMessageBody {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

impl AddDownstreamMessageBody {
    /// This is the request object to send messages out to the devices
    pub fn new() -> AddDownstreamMessageBody {
        AddDownstreamMessageBody { payload: None }
    }
}
