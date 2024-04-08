use super::definition::TickType;

pub enum SubscriptionType {
    QuoteData {
        tick_types: Vec<TickType>,
        conid: i64,
    },
    HistoricalData,
    MarketDepth,
    Orders,
    Positions,
    Trades,
    ProfitLoss,
}
