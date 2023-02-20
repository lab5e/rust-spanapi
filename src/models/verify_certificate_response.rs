/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.0 lean-joline
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// VerifyCertificateResponse : Response when verifying a certificate. The valid flag is set to true when the certificate is valid. Any errors will be added to the errors array.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifyCertificateResponse {
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

impl VerifyCertificateResponse {
    /// Response when verifying a certificate. The valid flag is set to true when the certificate is valid. Any errors will be added to the errors array.
    pub fn new() -> VerifyCertificateResponse {
        VerifyCertificateResponse {
            valid: None,
            errors: None,
        }
    }
}
