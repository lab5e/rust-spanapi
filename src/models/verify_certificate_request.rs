/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.6.1 squirming-codi
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// VerifyCertificateRequest : Verify a certificate

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifyCertificateRequest {
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}

impl VerifyCertificateRequest {
    /// Verify a certificate
    pub fn new() -> VerifyCertificateRequest {
        VerifyCertificateRequest {
            gateway_id: None,
            device_id: None,
            certificate: None,
        }
    }
}
