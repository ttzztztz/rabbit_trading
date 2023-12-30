use super::market::Market;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Symbol {
    pub market: Market,
    pub identifier: String,
}

impl std::string::ToString for Symbol {
    fn to_string(&self) -> String {
        format!("{}.{}", self.identifier, self.market.to_string())
    }
}
