/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 5.0.3 pitch-dark-elza
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// OutputType : Output types available   - undefined: The undefined output type is an invalid type

/// Output types available   - undefined: The undefined output type is an invalid type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutputType {
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "webhook")]
    Webhook,
    #[serde(rename = "udpout")]
    Udpout,
    #[serde(rename = "mqttclient")]
    Mqttclient,
    #[serde(rename = "ifttt")]
    Ifttt,
    #[serde(rename = "mqttbroker")]
    Mqttbroker,
}

impl ToString for OutputType {
    fn to_string(&self) -> String {
        match self {
            Self::Undefined => String::from("undefined"),
            Self::Webhook => String::from("webhook"),
            Self::Udpout => String::from("udpout"),
            Self::Mqttclient => String::from("mqttclient"),
            Self::Ifttt => String::from("ifttt"),
            Self::Mqttbroker => String::from("mqttbroker"),
        }
    }
}

impl Default for OutputType {
    fn default() -> OutputType {
        Self::Undefined
    }
}
