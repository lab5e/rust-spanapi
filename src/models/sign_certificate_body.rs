/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.3 pitch-dark-elza
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// SignCertificateBody : Request certificate signing

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SignCertificateBody {
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "csr", skip_serializing_if = "Option::is_none")]
    pub csr: Option<String>,
}

impl SignCertificateBody {
    /// Request certificate signing
    pub fn new() -> SignCertificateBody {
        SignCertificateBody {
            gateway_id: None,
            device_id: None,
            csr: None,
        }
    }
}