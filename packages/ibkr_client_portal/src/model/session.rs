use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tickle {
    pub collission: bool,
    pub iserver: Iserver,
    pub session: String,
    pub sso_expires: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Iserver {
    pub auth_status: AuthStatus,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    #[serde(rename = "MAC")]
    pub mac: Option<String>,
    /// Brokerage session is authenticated
    pub authenticated: Option<bool>,
    /// Brokerage session is competing, e.g. user is logged in to IBKR Mobile, WebTrader, TWS or other trading platforms.
    pub competing: Option<bool>,
    /// Connected to backend
    pub connected: Option<bool>,
    /// System messages that may affect trading
    pub message: Option<String>,
    pub server_info: Option<ServerInfo>,
    /// Authentication failed, why.
    pub fail: Option<String>,
    /// Prompt messages that may affect trading or the account
    pub prompts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub server_name: String,
    pub server_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LogoutResponse {
    /// true means username is still logged in, false means it is not
    pub confirmed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SSOValidateResponse {
    /// 1 for Live, 2 for Paper
    #[serde(rename = "LOGIN_TYPE")]
    pub login_type: Option<i64>,
    /// Username
    #[serde(rename = "USER_NAME")]
    pub user_name: Option<String>,
    /// User ID
    #[serde(rename = "USER_ID")]
    pub user_id: Option<i64>,
    /// Time in milliseconds until session expires. Caller needs to call the again to re-validate session
    #[serde(rename = "expire")]
    pub expire: Option<i64>,
    /// true if session was validated; false if not.
    #[serde(rename = "RESULT")]
    pub result: Option<bool>,
    /// Time of session validation
    #[serde(rename = "AUTH_TIME")]
    pub auth_time: Option<i64>,
}
