/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.7 only-deshaun
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemInfoResponse {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "buildDate", skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    #[serde(rename = "releaseName", skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    #[serde(rename = "defaultFieldMask", skip_serializing_if = "Option::is_none")]
    pub default_field_mask: Option<Box<crate::models::FieldMask>>,
    #[serde(rename = "forcedFieldMask", skip_serializing_if = "Option::is_none")]
    pub forced_field_mask: Option<Box<crate::models::FieldMask>>,
}

impl SystemInfoResponse {
    pub fn new() -> SystemInfoResponse {
        SystemInfoResponse {
            version: None,
            build_date: None,
            release_name: None,
            default_field_mask: None,
            forced_field_mask: None,
        }
    }
}


