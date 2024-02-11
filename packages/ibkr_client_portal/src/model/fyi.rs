use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFYIUnreadNumberResponse {
    /// unread number
    #[serde(rename = "BN")]
    pub bn: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFYISettingsResponse {
    /// optional, if A doesn't exist, it means user can't toggle this option. 0-off, 1-on.
    #[serde(rename = "A")]
    pub A: Option<i32>,
    /// fyi code
    #[serde(rename = "FC")]
    pub fyi_code: Option<String>,
    /// disclaimer read, 1 = yes, = 0 no.
    #[serde(rename = "H")]
    pub disclaimer_read: Option<i32>,
    /// detailed description
    #[serde(rename = "FD")]
    pub detailed_description: Option<String>,
    /// title
    #[serde(rename = "FN")]
    pub title: Option<String>,
}

pub struct GetFYIDisclaimerInfoRequest {
    pub type_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFYIDisclaimerInfoResponse {
    /// disclaimer message
    #[serde(rename = "DT")]
    pub disclaimer_message: Option<String>,
    /// fyi code
    #[serde(rename = "FC")]
    pub fyi_coed: Option<String>,
}
