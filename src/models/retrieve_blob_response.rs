/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.3 pitch-dark-elza
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// RetrieveBlobResponse : This is not available throught the API, just as a regular HTTP response

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RetrieveBlobResponse {
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

impl RetrieveBlobResponse {
    /// This is not available throught the API, just as a regular HTTP response
    pub fn new() -> RetrieveBlobResponse {
        RetrieveBlobResponse {
            content_type: None,
            size: None,
            data: None,
        }
    }
}
