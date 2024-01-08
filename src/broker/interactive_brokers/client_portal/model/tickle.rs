use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Tickle {
    pub collission: bool,
    pub iserver: Iserver,
    pub session: String,
    pub sso_expires: i64,
    pub user_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iserver {
    pub auth_status: AuthStatus,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct AuthStatus {
    #[serde(rename = "MAC")]
    pub mac: String,
    pub authenticated: bool,
    pub competing: bool,
    pub connected: bool,
    pub message: String,
    pub server_info: ServerInfo,
    pub fail: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct ServerInfo {
    pub server_name: String,
    pub server_version: String,
}
