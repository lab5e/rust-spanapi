/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.2 nonviolent-adelbert
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// Blob : This is a blob (binary large object) that the devices might upload to the service. This is messages that are typically too large to handle like regular status and sensor values, typically media files. The content type might be derived from the first few bytes of the blob and could possibly be incorrect.  Download the blob by accessing the blob URL field. This will work like a regular HTTP request for your client. Authentication is required as always.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Blob {
    #[serde(rename = "blobId", skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,
    #[serde(rename = "blobPath", skip_serializing_if = "Option::is_none")]
    pub blob_path: Option<String>,
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "gatewayId", skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}

impl Blob {
    /// This is a blob (binary large object) that the devices might upload to the service. This is messages that are typically too large to handle like regular status and sensor values, typically media files. The content type might be derived from the first few bytes of the blob and could possibly be incorrect.  Download the blob by accessing the blob URL field. This will work like a regular HTTP request for your client. Authentication is required as always.
    pub fn new() -> Blob {
        Blob {
            blob_id: None,
            blob_path: None,
            content_type: None,
            size: None,
            created: None,
            collection_id: None,
            device_id: None,
            gateway_id: None,
            properties: None,
        }
    }
}


