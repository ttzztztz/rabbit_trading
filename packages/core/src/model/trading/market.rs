use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Market {
    CN,
    HK,
    US,
}

impl std::string::ToString for Market {
    fn to_string(&self) -> String {
        match self {
            Market::CN => String::from("CN"),
            Market::HK => String::from("HK"),
            Market::US => String::from("US"),
        }
    }
}

impl FromStr for Market {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "US" => Result::Ok(Market::US),
            "HK" => Result::Ok(Market::HK),
            "CN" => Result::Ok(Market::CN),
            _ => Result::Err(anyhow!("PARSING_ERROR Error when parsing market {}", s)),
        }
    }
}
