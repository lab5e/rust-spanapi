/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.2 nonviolent-adelbert
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CertificateChainResponse : Response when retrieving a certificate chain



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CertificateChainResponse {
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
}

impl CertificateChainResponse {
    /// Response when retrieving a certificate chain
    pub fn new() -> CertificateChainResponse {
        CertificateChainResponse {
            chain: None,
        }
    }
}


