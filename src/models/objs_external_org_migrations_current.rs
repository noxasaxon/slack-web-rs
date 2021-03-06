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
pub struct ObjsExternalOrgMigrationsCurrent {
    #[serde(rename = "date_started")]
    pub date_started: i32,
    #[serde(rename = "team_id")]
    pub team_id: String,
}

impl ObjsExternalOrgMigrationsCurrent {
    pub fn new(date_started: i32, team_id: String) -> ObjsExternalOrgMigrationsCurrent {
        ObjsExternalOrgMigrationsCurrent {
            date_started,
            team_id,
        }
    }
}


