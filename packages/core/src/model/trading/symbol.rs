use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::market::Market;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Symbol {
    pub market: Market,
    pub identifier: String,
}

impl std::string::ToString for Symbol {
    fn to_string(&self) -> String {
        format!("{}.{}", self.identifier, self.market.to_string())
    }
}

impl FromStr for Symbol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted_vec: Vec<&str> = s.split('.').collect();
        if splitted_vec.len() != 2 {
            return Result::Err(anyhow!("PARSING_ERROR Error when parsing symbol {}", s));
        }

        Result::Ok(Symbol {
            market: splitted_vec[1].parse()?,
            identifier: splitted_vec[0].to_owned(),
        })
    }
}
