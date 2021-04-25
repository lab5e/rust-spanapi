/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.10 hulking-betty
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearFirmwareErrorResponse {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl ClearFirmwareErrorResponse {
    pub fn new() -> ClearFirmwareErrorResponse {
        ClearFirmwareErrorResponse {
            result: None,
        }
    }
}


