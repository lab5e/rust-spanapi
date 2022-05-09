/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.2.3 lower-elian
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CoApMetadata : CoAP metadata for messages received through one of the CoAP endpoints



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CoApMetadata {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl CoApMetadata {
    /// CoAP metadata for messages received through one of the CoAP endpoints
    pub fn new() -> CoApMetadata {
        CoApMetadata {
            code: None,
            path: None,
        }
    }
}


