/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.8.0 indecipherable-ferman
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// DeviceCertificateResponse : Response object for certificate info resource

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeviceCertificateResponse {
    #[serde(rename = "certificates", skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<crate::models::CertificateInfo>>,
}

impl DeviceCertificateResponse {
    /// Response object for certificate info resource
    pub fn new() -> DeviceCertificateResponse {
        DeviceCertificateResponse { certificates: None }
    }
}
