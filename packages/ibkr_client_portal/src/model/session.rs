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

#[derive(Debug, Serialize, Deserialize)]
pub struct SSOValidateResponse {
    /// 1 for Live, 2 for Paper
    #[serde(rename = "LOGIN_TYPE")]
    pub login_type: Option<f32>,
    /// Username
    #[serde(rename = "USER_NAME")]
    pub user_name: Option<String>,
    /// User ID
    #[serde(rename = "USER_ID")]
    pub user_id: Option<f32>,
    /// Time in milliseconds until session expires. Caller needs to call the again to re-validate session
    #[serde(rename = "expire")]
    pub expire: Option<f32>,
    /// true if session was validated; false if not.
    #[serde(rename = "RESULT")]
    pub result: Option<bool>,
    /// Time of session validation
    #[serde(rename = "AUTH_TIME")]
    pub auth_time: Option<f32>,
}
