/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.7.0 actionable-aryanna
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CertificateInfo : Certificate information

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CertificateInfo {
    #[serde(rename = "certificateSerial", skip_serializing_if = "Option::is_none")]
    pub certificate_serial: Option<String>,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
}

impl CertificateInfo {
    /// Certificate information
    pub fn new() -> CertificateInfo {
        CertificateInfo {
            certificate_serial: None,
            expires: None,
        }
    }
}
