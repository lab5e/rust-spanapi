/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.6.1 squirming-codi
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// OutputStats : Statistics for a single data router

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OutputStats {
    #[serde(rename = "forwardErrors", skip_serializing_if = "Option::is_none")]
    pub forward_errors: Option<i32>,
    #[serde(rename = "messagesForwarded", skip_serializing_if = "Option::is_none")]
    pub messages_forwarded: Option<i32>,
    #[serde(rename = "bytesForwardedMb", skip_serializing_if = "Option::is_none")]
    pub bytes_forwarded_mb: Option<f32>,
}

impl OutputStats {
    /// Statistics for a single data router
    pub fn new() -> OutputStats {
        OutputStats {
            forward_errors: None,
            messages_forwarded: None,
            bytes_forwarded_mb: None,
        }
    }
}
