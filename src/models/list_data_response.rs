/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.0 lean-joline
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListDataResponse : List of device payloads



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListDataResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::OutputDataMessage>>,
}

impl ListDataResponse {
    /// List of device payloads
    pub fn new() -> ListDataResponse {
        ListDataResponse {
            data: None,
        }
    }
}


