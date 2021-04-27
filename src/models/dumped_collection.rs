/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.12 infinite-dana
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DumpedCollection {
    #[serde(rename = "collection", skip_serializing_if = "Option::is_none")]
    pub collection: Option<Box<crate::models::Collection>>,
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::DumpedDevice>>,
    #[serde(rename = "outputs", skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<crate::models::Output>>,
}

impl DumpedCollection {
    pub fn new() -> DumpedCollection {
        DumpedCollection {
            collection: None,
            devices: None,
            outputs: None,
        }
    }
}
