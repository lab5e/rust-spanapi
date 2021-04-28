/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.13 interdependent-karson
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutputType {
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "webhook")]
    Webhook,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "mqtt")]
    Mqtt,
    #[serde(rename = "ifttt")]
    Ifttt,
}

impl ToString for OutputType {
    fn to_string(&self) -> String {
        match self {
            Self::Undefined => String::from("undefined"),
            Self::Webhook => String::from("webhook"),
            Self::Udp => String::from("udp"),
            Self::Mqtt => String::from("mqtt"),
            Self::Ifttt => String::from("ifttt"),
        }
    }
}
