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
pub struct ObjsReaction {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "users")]
    pub users: Vec<String>,
}

impl ObjsReaction {
    pub fn new(count: i32, name: String, users: Vec<String>) -> ObjsReaction {
        ObjsReaction {
            count,
            name,
            users,
        }
    }
}

