/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjsUserProfileShort {
    #[serde(rename = "avatar_hash")]
    pub avatar_hash: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "display_name_normalized", skip_serializing_if = "Option::is_none")]
    pub display_name_normalized: Option<String>,
    #[serde(rename = "image_72")]
    pub image_72: String,
    #[serde(rename = "is_restricted")]
    pub is_restricted: bool,
    #[serde(rename = "is_ultra_restricted")]
    pub is_ultra_restricted: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "real_name")]
    pub real_name: String,
    #[serde(rename = "real_name_normalized", skip_serializing_if = "Option::is_none")]
    pub real_name_normalized: Option<String>,
    #[serde(rename = "team")]
    pub team: String,
}

impl ObjsUserProfileShort {
    pub fn new(avatar_hash: String, display_name: String, image_72: String, is_restricted: bool, is_ultra_restricted: bool, name: String, real_name: String, team: String) -> ObjsUserProfileShort {
        ObjsUserProfileShort {
            avatar_hash,
            display_name,
            display_name_normalized: None,
            image_72,
            is_restricted,
            is_ultra_restricted,
            name,
            real_name,
            real_name_normalized: None,
            team,
        }
    }
}


