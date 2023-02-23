/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.1 busy-janay
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// MessageTransport : The message transport can be UDP or CoAP.

/// The message transport can be UDP or CoAP.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageTransport {
    #[serde(rename = "unspecified")]
    Unspecified,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "coap")]
    Coap,
    #[serde(rename = "mqtt")]
    Mqtt,
    #[serde(rename = "gateway")]
    Gateway,
    #[serde(rename = "coaps")]
    Coaps,
    #[serde(rename = "dtls")]
    Dtls,
}

impl ToString for MessageTransport {
    fn to_string(&self) -> String {
        match self {
            Self::Unspecified => String::from("unspecified"),
            Self::Udp => String::from("udp"),
            Self::Coap => String::from("coap"),
            Self::Mqtt => String::from("mqtt"),
            Self::Gateway => String::from("gateway"),
            Self::Coaps => String::from("coaps"),
            Self::Dtls => String::from("dtls"),
        }
    }
}

impl Default for MessageTransport {
    fn default() -> MessageTransport {
        Self::Unspecified
    }
}
