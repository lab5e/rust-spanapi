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
pub struct FieldMask {
    #[serde(rename = "imsi", skip_serializing_if = "Option::is_none")]
    pub imsi: Option<bool>,
    #[serde(rename = "imei", skip_serializing_if = "Option::is_none")]
    pub imei: Option<bool>,
    #[serde(rename = "msisdn", skip_serializing_if = "Option::is_none")]
    pub msisdn: Option<bool>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<bool>,
}

impl FieldMask {
    pub fn new() -> FieldMask {
        FieldMask {
            imsi: None,
            imei: None,
            msisdn: None,
            location: None,
        }
    }
}
