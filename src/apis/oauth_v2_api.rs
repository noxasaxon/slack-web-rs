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


/// struct for typed errors of method `oauth_v2_access`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OauthV2AccessError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Exchanges a temporary OAuth verifier code for an access token.
pub async fn oauth_v2_access(configuration: &configuration::Configuration, code: &str, client_id: Option<&str>, client_secret: Option<&str>, redirect_uri: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<OauthV2AccessError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/oauth.v2.access", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = client_id {
        local_var_req_builder = local_var_req_builder.query(&[("client_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = client_secret {
        local_var_req_builder = local_var_req_builder.query(&[("client_secret", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("code", &code.to_string())]);
    if let Some(ref local_var_str) = redirect_uri {
        local_var_req_builder = local_var_req_builder.query(&[("redirect_uri", &local_var_str.to_string())]);
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
        let local_var_entity: Option<OauthV2AccessError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
