/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.15 disproved-darryl
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoApMetadata {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl CoApMetadata {
    pub fn new() -> CoApMetadata {
        CoApMetadata {
            code: None,
            path: None,
        }
    }
}
