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
pub struct DataDumpResponse {
    #[serde(rename = "collections", skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<crate::models::DumpedCollection>>,
}

impl DataDumpResponse {
    pub fn new() -> DataDumpResponse {
        DataDumpResponse {
            collections: None,
        }
    }
}


