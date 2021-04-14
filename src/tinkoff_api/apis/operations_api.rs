/*
 * OpenAPI
 *
 * tinkoff.ru/invest OpenAPI.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: al.a.volkov@tinkoff.ru
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::tinkoff_api::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `operations_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OperationsGetError {
    Status500(crate::tinkoff_api::models::Error),
    UnknownValue(serde_json::Value),
}


pub async fn operations_get(configuration: &configuration::Configuration, from: String, to: String, figi: Option<&str>, broker_account_id: Option<&str>) -> Result<crate::tinkoff_api::models::OperationsResponse, Error<OperationsGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/operations", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("from", &from.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("to", &to.to_string())]);
    if let Some(ref local_var_str) = figi {
        local_var_req_builder = local_var_req_builder.query(&[("figi", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = broker_account_id {
        local_var_req_builder = local_var_req_builder.query(&[("brokerAccountId", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OperationsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

