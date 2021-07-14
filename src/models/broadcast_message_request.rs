/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.16 spooky-devante
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BroadcastMessageRequest {
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    /// Valid transports are \"udp\", \"coap\", \"coap-pull\", \"udp-pull\", \"coap-push\", \"udp-push\". \"udp\" is equivalent to \"udp-push\" and \"coap\" is equivalent to \"coap-push\". Push messages are sent unsolicited to the device wheil pull messages are sent whenever the device wither sends data upstream (for UDP) or does a CoAP request to the CoAP service in span.
    #[serde(rename = "transport", skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
    #[serde(rename = "coapPath", skip_serializing_if = "Option::is_none")]
    pub coap_path: Option<String>,
}

impl BroadcastMessageRequest {
    pub fn new() -> BroadcastMessageRequest {
        BroadcastMessageRequest {
            collection_id: None,
            port: None,
            payload: None,
            transport: None,
            coap_path: None,
        }
    }
}


