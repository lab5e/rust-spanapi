/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.1 busy-janay
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GatewayInetConfig {
    #[serde(rename = "dtlsEndpoint", skip_serializing_if = "Option::is_none")]
    pub dtls_endpoint: Option<String>,
    #[serde(rename = "coapEndpoint", skip_serializing_if = "Option::is_none")]
    pub coap_endpoint: Option<String>,
    #[serde(rename = "mqttEndpoint", skip_serializing_if = "Option::is_none")]
    pub mqtt_endpoint: Option<String>,
}

impl GatewayInetConfig {
    pub fn new() -> GatewayInetConfig {
        GatewayInetConfig {
            dtls_endpoint: None,
            coap_endpoint: None,
            mqtt_endpoint: None,
        }
    }
}


