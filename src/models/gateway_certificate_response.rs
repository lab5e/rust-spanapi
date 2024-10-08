/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.3 pitch-dark-elza
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GatewayCertificateResponse {
    #[serde(rename = "certificates", skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<crate::models::CertificateInfo>>,
}

impl GatewayCertificateResponse {
    pub fn new() -> GatewayCertificateResponse {
        GatewayCertificateResponse { certificates: None }
    }
}
