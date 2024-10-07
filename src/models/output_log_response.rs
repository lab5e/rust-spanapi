/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.3 pitch-dark-elza
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// OutputLogResponse : List logs for output

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OutputLogResponse {
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<crate::models::OutputLogEntry>>,
}

impl OutputLogResponse {
    /// List logs for output
    pub fn new() -> OutputLogResponse {
        OutputLogResponse { logs: None }
    }
}
