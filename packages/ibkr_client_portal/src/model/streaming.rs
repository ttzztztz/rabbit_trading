use serde_json::json;

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

#[deprecated]
pub struct Subscription {
    pub sub_type: SubscriptionType,
    pub exchange: Option<String>,
}

impl Subscription {
    pub fn new_smart_quote_data(conid: i64) -> Self {
        let tick_types = vec![
            TickType::BidPrice,
            TickType::AskPrice,
            TickType::AskSize,
            TickType::BidSize,
            TickType::LastPrice,
        ];
        Self {
            sub_type: SubscriptionType::QuoteData { tick_types, conid },
            exchange: None,
        }
    }

    pub fn build(&self) -> String {
        match &self.sub_type {
            SubscriptionType::QuoteData { tick_types, conid } => {
                let field_list = tick_types
                    .iter()
                    .map(|t| serde_json::to_string(t).unwrap())
                    .collect::<Vec<String>>();
                let contract_rep = match &self.exchange {
                    Some(exchange) => format!("{}@{exchange}", conid),
                    None => conid.to_string(),
                };
                let arg_json = json!({ "fields": field_list });
                format!("smd+{contract_rep}+{}", arg_json)
            }
            SubscriptionType::HistoricalData => unimplemented!(),
            SubscriptionType::MarketDepth => unimplemented!(),
            SubscriptionType::Orders => unimplemented!(),
            SubscriptionType::Positions => unimplemented!(),
            SubscriptionType::Trades => String::from("str+{}"),
            SubscriptionType::ProfitLoss => unimplemented!(),
        }
    }
}
