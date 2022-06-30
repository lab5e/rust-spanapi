/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.2.4 curable-andres
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

/// ListNetworkResponse : List networks.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListNetworkResponse {
    #[serde(rename = "networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::Network>>,
}

impl ListNetworkResponse {
    /// List networks.
    pub fn new() -> ListNetworkResponse {
        ListNetworkResponse { networks: None }
    }
}
