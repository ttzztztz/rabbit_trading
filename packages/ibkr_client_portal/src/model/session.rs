use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tickle {
    pub collission: bool,
    pub iserver: Iserver,
    pub session: String,
    pub sso_expires: i64,
    pub user_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iserver {
    pub auth_status: AuthStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    #[serde(rename = "MAC")]
    pub mac: String,
    pub authenticated: bool,
    pub competing: bool,
    pub connected: bool,
    pub message: Option<String>,
    pub server_info: Option<ServerInfo>,
    pub fail: Option<String>,
    pub prompts: Option<Vec<String>>,    
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub server_name: String,
    pub server_version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub confirmed: bool,
}
