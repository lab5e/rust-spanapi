/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.4.0 lean-joline
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MessageState {
    #[serde(rename = "unspecified")]
    Unspecified,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "failed")]
    Failed,

}

impl ToString for MessageState {
    fn to_string(&self) -> String {
        match self {
            Self::Unspecified => String::from("unspecified"),
            Self::Pending => String::from("pending"),
            Self::Sent => String::from("sent"),
            Self::Failed => String::from("failed"),
        }
    }
}

impl Default for MessageState {
    fn default() -> MessageState {
        Self::Unspecified
    }
}




