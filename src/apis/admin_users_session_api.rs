/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `admin_users_session_invalidate`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdminUsersSessionInvalidateError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `admin_users_session_reset`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdminUsersSessionResetError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Invalidate a single session for a user by session_id
pub async fn admin_users_session_invalidate(configuration: &configuration::Configuration, token: &str, team_id: &str, session_id: i32) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AdminUsersSessionInvalidateError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/admin.users.session.invalidate", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("team_id", team_id.to_string());
    local_var_form_params.insert("session_id", session_id.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AdminUsersSessionInvalidateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Wipes all valid sessions on all devices for a given user
pub async fn admin_users_session_reset(configuration: &configuration::Configuration, token: &str, user_id: &str, mobile_only: Option<bool>, web_only: Option<bool>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AdminUsersSessionResetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/admin.users.session.reset", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("user_id", user_id.to_string());
    if let Some(local_var_param_value) = mobile_only {
        local_var_form_params.insert("mobile_only", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = web_only {
        local_var_form_params.insert("web_only", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AdminUsersSessionResetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

