/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.14 oversensitive-deante
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutputDataMessageOutputMessageType {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "keepalive")]
    Keepalive,
    #[serde(rename = "data")]
    Data,

}

impl ToString for OutputDataMessageOutputMessageType {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("unknown"),
            Self::Keepalive => String::from("keepalive"),
            Self::Data => String::from("data"),
        }
    }
}




