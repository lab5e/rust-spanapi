/*
 * The Span API
 *
 * API for device, collection, output and firmware management
 *
 * The version of the OpenAPI document: 4.1.8 adopted-kali
 * Contact: dev@lab5e.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method `data_dump`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataDumpError {
    Status400(serde_json::Value),
    Status401(serde_json::Value),
    Status404(serde_json::Value),
    Status409(serde_json::Value),
    Status500(serde_json::Value),
    DefaultResponse(crate::models::RpcStatus),
    UnknownValue(serde_json::Value),
}

/// Do a complete data dump of your data, devices, outputs and collections.
pub async fn data_dump(
    configuration: &configuration::Configuration,
    body: crate::models::DataDumpRequest,
) -> Result<crate::models::DataDumpResponse, Error<DataDumpError>> {
    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/datadump", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DataDumpError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
