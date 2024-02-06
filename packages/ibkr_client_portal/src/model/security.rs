use serde::{Deserialize, Serialize};

use super::definition::AssetClass;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchForSecurityRequest {
    pub symbol: String,
    #[serde(rename = "name")]
    pub is_name: bool,
    #[serde(rename = "secType")]
    pub sec_type: AssetClass,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecurityDefinitionByContractIdRequest {
    conids: Vec<i64>,
}
