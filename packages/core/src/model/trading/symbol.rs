use serde::{Deserialize, Serialize};

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
