/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.0 convulsive-launa
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// SystemInfoResponse : Response object for system information. This contains system-level information.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SystemInfoResponse {
    /// This is the system version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The build time for this version.
    #[serde(rename = "buildDate", skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    /// Human-readable code name for this release. This can be easier to remember than the version number.
    #[serde(rename = "releaseName", skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
}

impl SystemInfoResponse {
    /// Response object for system information. This contains system-level information.
    pub fn new() -> SystemInfoResponse {
        SystemInfoResponse {
            version: None,
            build_date: None,
            release_name: None,
        }
    }
}
