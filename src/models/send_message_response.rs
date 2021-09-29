/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.18 disgruntled-jerald
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMessageResponse {
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "bytesSent", skip_serializing_if = "Option::is_none")]
    pub bytes_sent: Option<i32>,
}

impl SendMessageResponse {
    pub fn new() -> SendMessageResponse {
        SendMessageResponse {
            collection_id: None,
            device_id: None,
            bytes_sent: None,
        }
    }
}


