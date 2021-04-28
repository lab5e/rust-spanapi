/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.13 interdependent-karson
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtobufAny {
    #[serde(rename = "typeUrl", skip_serializing_if = "Option::is_none")]
    pub type_url: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ProtobufAny {
    pub fn new() -> ProtobufAny {
        ProtobufAny {
            type_url: None,
            value: None,
        }
    }
}
