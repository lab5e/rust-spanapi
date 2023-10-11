/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.7.0 actionable-aryanna
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateCertificateRequest : Request object to create a new certificate.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateCertificateRequest {
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

impl CreateCertificateRequest {
    /// Request object to create a new certificate.
    pub fn new() -> CreateCertificateRequest {
        CreateCertificateRequest {
            gateway_id: None,
            device_id: None,
        }
    }
}
