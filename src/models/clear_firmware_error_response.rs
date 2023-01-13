/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.3.0 grouchy-aloysius
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ClearFirmwareErrorResponse : Clear firmware error response object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearFirmwareErrorResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl ClearFirmwareErrorResponse {
    /// Clear firmware error response object
    pub fn new() -> ClearFirmwareErrorResponse {
        ClearFirmwareErrorResponse { result: None }
    }
}
