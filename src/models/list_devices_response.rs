/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.7 prized-adelbert
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListDevicesResponse {
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::Device>>,
}

impl ListDevicesResponse {
    pub fn new() -> ListDevicesResponse {
        ListDevicesResponse {
            devices: None,
        }
    }
}


