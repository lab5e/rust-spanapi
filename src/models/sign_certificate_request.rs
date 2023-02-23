/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.1 busy-janay
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// SignCertificateRequest : Request certificate signing

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignCertificateRequest {
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "csr", skip_serializing_if = "Option::is_none")]
    pub csr: Option<String>,
}

impl SignCertificateRequest {
    /// Request certificate signing
    pub fn new() -> SignCertificateRequest {
        SignCertificateRequest {
            gateway_id: None,
            device_id: None,
            csr: None,
        }
    }
}
