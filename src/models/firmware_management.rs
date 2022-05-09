/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.2.3 lower-elian
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// FirmwareManagement : The firmware management settings for a collection can either be \"disabled\", ie there is no firmware management for this collection, \"collection\"; devices are managed through the settings on the collection or \"device\" where each device is configured individual.

/// The firmware management settings for a collection can either be \"disabled\", ie there is no firmware management for this collection, \"collection\"; devices are managed through the settings on the collection or \"device\" where each device is configured individual.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FirmwareManagement {
    #[serde(rename = "unspecified")]
    Unspecified,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "collection")]
    Collection,
    #[serde(rename = "device")]
    Device,
}

impl ToString for FirmwareManagement {
    fn to_string(&self) -> String {
        match self {
            Self::Unspecified => String::from("unspecified"),
            Self::Disabled => String::from("disabled"),
            Self::Collection => String::from("collection"),
            Self::Device => String::from("device"),
        }
    }
}

impl Default for FirmwareManagement {
    fn default() -> FirmwareManagement {
        Self::Unspecified
    }
}
