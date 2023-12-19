use super::market::Market;

#[derive(Clone, Debug)]
pub struct Symbol {
    pub market: Market,
    pub identifier: String,
}

impl std::string::ToString for Symbol {
    fn to_string(&self) -> String {
        format!("{}.{}", self.identifier, self.market.to_string())
    }
}
