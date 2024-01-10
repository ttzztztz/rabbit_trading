use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AssetClass {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "CFD")]
    Cfd,
    #[serde(rename = "FUT")]
    Future,
    #[serde(rename = "IND")]
    Index,
    #[serde(rename = "OPT")]
    Option,
    #[serde(rename = "STK")]
    Stock,
    #[serde(rename = "FOP")]
    FuturesOptions,
    #[serde(rename = "FUND")]
    MutualFund,
    #[serde(rename = "CMDTY")]
    Commodity,
    #[serde(rename = "unknown")]
    #[default]
    Unknown,
}

impl Display for AssetClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetClass::Bond => write!(f, "BOND"),
            AssetClass::Cfd => write!(f, "CFD"),
            AssetClass::Future => write!(f, "FUT"),
            AssetClass::Index => write!(f, "IND"),
            AssetClass::Option => write!(f, "OPT"),
            AssetClass::Stock => write!(f, "STK"),
            AssetClass::FuturesOptions => write!(f, "FOP"),
            AssetClass::MutualFund => write!(f, "FUND"),
            AssetClass::Commodity => write!(f, "CMDTY"),
            AssetClass::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptionRight {
    #[serde(rename = "C")]
    Call,
    #[serde(rename = "P")]
    Put,
}
