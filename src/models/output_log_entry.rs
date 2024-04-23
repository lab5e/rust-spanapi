/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.9.5 spattered-kelvin
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// OutputLogEntry : Log entries for outputs

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OutputLogEntry {
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "repeated", skip_serializing_if = "Option::is_none")]
    pub repeated: Option<i32>,
}

impl OutputLogEntry {
    /// Log entries for outputs
    pub fn new() -> OutputLogEntry {
        OutputLogEntry {
            time: None,
            message: None,
            repeated: None,
        }
    }
}
