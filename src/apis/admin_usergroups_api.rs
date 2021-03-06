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


/// struct for typed errors of method `admin_usergroups_add_channels`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdminUsergroupsAddChannelsError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `admin_usergroups_add_teams`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdminUsergroupsAddTeamsError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `admin_usergroups_list_channels`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdminUsergroupsListChannelsError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `admin_usergroups_remove_channels`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdminUsergroupsRemoveChannelsError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Add one or more default channels to an IDP group.
pub async fn admin_usergroups_add_channels(configuration: &configuration::Configuration, token: &str, usergroup_id: &str, channel_ids: &str, team_id: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AdminUsergroupsAddChannelsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/admin.usergroups.addChannels", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("usergroup_id", usergroup_id.to_string());
    if let Some(local_var_param_value) = team_id {
        local_var_form_params.insert("team_id", local_var_param_value.to_string());
    }
    local_var_form_params.insert("channel_ids", channel_ids.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AdminUsergroupsAddChannelsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Associate one or more default workspaces with an organization-wide IDP group.
pub async fn admin_usergroups_add_teams(configuration: &configuration::Configuration, token: &str, usergroup_id: &str, team_ids: &str, auto_provision: Option<bool>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AdminUsergroupsAddTeamsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/admin.usergroups.addTeams", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("usergroup_id", usergroup_id.to_string());
    local_var_form_params.insert("team_ids", team_ids.to_string());
    if let Some(local_var_param_value) = auto_provision {
        local_var_form_params.insert("auto_provision", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AdminUsergroupsAddTeamsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List the channels linked to an org-level IDP group (user group).
pub async fn admin_usergroups_list_channels(configuration: &configuration::Configuration, token: &str, usergroup_id: &str, team_id: Option<&str>, include_num_members: Option<bool>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AdminUsergroupsListChannelsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/admin.usergroups.listChannels", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("usergroup_id", &usergroup_id.to_string())]);
    if let Some(ref local_var_str) = team_id {
        local_var_req_builder = local_var_req_builder.query(&[("team_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_num_members {
        local_var_req_builder = local_var_req_builder.query(&[("include_num_members", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AdminUsergroupsListChannelsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove one or more default channels from an org-level IDP group (user group).
pub async fn admin_usergroups_remove_channels(configuration: &configuration::Configuration, token: &str, usergroup_id: &str, channel_ids: &str) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AdminUsergroupsRemoveChannelsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/admin.usergroups.removeChannels", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("usergroup_id", usergroup_id.to_string());
    local_var_form_params.insert("channel_ids", channel_ids.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AdminUsergroupsRemoveChannelsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

