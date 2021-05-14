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


/// struct for typed errors of method `users_conversations`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersConversationsError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_delete_photo`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersDeletePhotoError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_get_presence`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersGetPresenceError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_identity`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersIdentityError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_info`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersInfoError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_lookup_by_email`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersLookupByEmailError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_profile_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersProfileGetError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_profile_set`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersProfileSetError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_set_active`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersSetActiveError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_set_photo`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersSetPhotoError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `users_set_presence`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersSetPresenceError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// List conversations the calling user may access.
pub async fn users_conversations(configuration: &configuration::Configuration, token: Option<&str>, user: Option<&str>, types: Option<&str>, exclude_archived: Option<bool>, limit: Option<i32>, cursor: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersConversationsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.conversations", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = types {
        local_var_req_builder = local_var_req_builder.query(&[("types", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = exclude_archived {
        local_var_req_builder = local_var_req_builder.query(&[("exclude_archived", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
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
        let local_var_entity: Option<UsersConversationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the user profile photo
pub async fn users_delete_photo(configuration: &configuration::Configuration, token: &str) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersDeletePhotoError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.deletePhoto", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("token", token.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UsersDeletePhotoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets user presence information.
pub async fn users_get_presence(configuration: &configuration::Configuration, token: &str, user: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersGetPresenceError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.getPresence", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
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
        let local_var_entity: Option<UsersGetPresenceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a user's identity.
pub async fn users_identity(configuration: &configuration::Configuration, token: Option<&str>) -> Result<serde_json::Value, Error<UsersIdentityError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.identity", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
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
        let local_var_entity: Option<UsersIdentityError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets information about a user.
pub async fn users_info(configuration: &configuration::Configuration, token: &str, include_locale: Option<bool>, user: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersInfoError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.info", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_str) = include_locale {
        local_var_req_builder = local_var_req_builder.query(&[("include_locale", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
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
        let local_var_entity: Option<UsersInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all users in a Slack team.
pub async fn users_list(configuration: &configuration::Configuration, token: Option<&str>, limit: Option<i32>, cursor: Option<&str>, include_locale: Option<bool>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersListError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.list", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_locale {
        local_var_req_builder = local_var_req_builder.query(&[("include_locale", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
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
        let local_var_entity: Option<UsersListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Find a user with an email address.
pub async fn users_lookup_by_email(configuration: &configuration::Configuration, token: &str, email: &str) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersLookupByEmailError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.lookupByEmail", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("email", &email.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
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
        let local_var_entity: Option<UsersLookupByEmailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a user's profile information.
pub async fn users_profile_get(configuration: &configuration::Configuration, token: &str, include_labels: Option<bool>, user: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersProfileGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.profile.get", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_str) = include_labels {
        local_var_req_builder = local_var_req_builder.query(&[("include_labels", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user {
        local_var_req_builder = local_var_req_builder.query(&[("user", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
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
        let local_var_entity: Option<UsersProfileGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set the profile information for a user.
pub async fn users_profile_set(configuration: &configuration::Configuration, token: &str, name: Option<&str>, profile: Option<&str>, user: Option<&str>, value: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersProfileSetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.profile.set", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = profile {
        local_var_form_params.insert("profile", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = user {
        local_var_form_params.insert("user", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = value {
        local_var_form_params.insert("value", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UsersProfileSetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Marked a user as active. Deprecated and non-functional.
pub async fn users_set_active(configuration: &configuration::Configuration, token: &str) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersSetActiveError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.setActive", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

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
        let local_var_entity: Option<UsersSetActiveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Set the user profile photo
pub async fn users_set_photo(configuration: &configuration::Configuration, token: &str, crop_w: Option<&str>, crop_x: Option<&str>, crop_y: Option<&str>, image: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersSetPhotoError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.setPhoto", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("token", token.to_string());
    if let Some(local_var_param_value) = crop_w {
        local_var_form_params.insert("crop_w", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = crop_x {
        local_var_form_params.insert("crop_x", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = crop_y {
        local_var_form_params.insert("crop_y", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = image {
        local_var_form_params.insert("image", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UsersSetPhotoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Manually sets user presence.
pub async fn users_set_presence(configuration: &configuration::Configuration, token: &str, presence: &str) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<UsersSetPresenceError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/users.setPresence", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    local_var_form_params.insert("presence", presence.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UsersSetPresenceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

