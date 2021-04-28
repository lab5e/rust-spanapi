/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.13 interdependent-karson
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputStatusResponse {
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "outputId", skip_serializing_if = "Option::is_none")]
    pub output_id: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "errorCount", skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i32>,
    #[serde(rename = "forwarded", skip_serializing_if = "Option::is_none")]
    pub forwarded: Option<i32>,
    #[serde(rename = "received", skip_serializing_if = "Option::is_none")]
    pub received: Option<i32>,
    #[serde(rename = "retransmits", skip_serializing_if = "Option::is_none")]
    pub retransmits: Option<i32>,
}

impl OutputStatusResponse {
    pub fn new() -> OutputStatusResponse {
        OutputStatusResponse {
            collection_id: None,
            output_id: None,
            enabled: None,
            error_count: None,
            forwarded: None,
            received: None,
            retransmits: None,
        }
    }
}
