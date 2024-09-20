/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.2 subversive-jamila
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// Firmware : Firmware images aren't served back out through the API, only the metadata.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Firmware {
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// Make sure that the firmware image reports this version. If the version reported by this image is different the FOTA process won't be reported as successful since the device will report a version different from this.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// This is just for internal house keeping, making it potentially easier to identify the firmware image.
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// To ensure each image is unique the SHA-256 hash for all images in a collection must be unqique
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    /// Collection ID for the collection owning the firmware image.
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Tags for firmware image.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Firmware {
    /// Firmware images aren't served back out through the API, only the metadata.
    pub fn new() -> Firmware {
        Firmware {
            image_id: None,
            version: None,
            filename: None,
            sha256: None,
            length: None,
            collection_id: None,
            created: None,
            tags: None,
            enabled: None,
        }
    }
}
