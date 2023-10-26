/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.8.0 indecipherable-ferman
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateCertificateResponse : Response when creating a new certificate

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCertificateResponse {
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
}

impl CreateCertificateResponse {
    /// Response when creating a new certificate
    pub fn new() -> CreateCertificateResponse {
        CreateCertificateResponse {
            certificate: None,
            private_key: None,
            chain: None,
        }
    }
}
