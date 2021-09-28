/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.17 enhanced-allie
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputLogEntry {
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "repeated", skip_serializing_if = "Option::is_none")]
    pub repeated: Option<i32>,
}

impl OutputLogEntry {
    pub fn new() -> OutputLogEntry {
        OutputLogEntry {
            time: None,
            message: None,
            repeated: None,
        }
    }
}


