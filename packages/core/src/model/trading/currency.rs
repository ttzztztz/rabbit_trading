use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Hash)]
pub enum Currency {
    CNH,
    CNY,
    HKD,
    JPY,
    USD,
}

impl FromStr for Currency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "HKD" => Result::Ok(Currency::HKD),
            "USD" => Result::Ok(Currency::USD),
            "CNH" => Result::Ok(Currency::CNH),
            _ => Result::Err(anyhow!("PARSING_ERROR Error when parsing currency {}", s)),
        }
    }
}
