/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.2 subversive-jamila
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateOutputBody : Request type when creating new outputs

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateOutputBody {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::OutputType>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::OutputConfig>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl CreateOutputBody {
    /// Request type when creating new outputs
    pub fn new() -> CreateOutputBody {
        CreateOutputBody {
            r#type: None,
            config: None,
            enabled: None,
            tags: None,
        }
    }
}
