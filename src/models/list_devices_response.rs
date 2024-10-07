/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.3 pitch-dark-elza
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListDevicesResponse : List device response

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListDevicesResponse {
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::Device>>,
}

impl ListDevicesResponse {
    /// List device response
    pub fn new() -> ListDevicesResponse {
        ListDevicesResponse { devices: None }
    }
}
