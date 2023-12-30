use async_trait::async_trait;
use time::{format_description, OffsetDateTime};
use tokio::select;

use crate::{
    model::{
        market::Market,
        quote::{QueryInfoRequest, QuoteKind},
        symbol::Symbol,
    },
    strategy::common::strategy::{StrategyContext, StrategyTrait},
};

pub struct PrintLivePriceStrategy {
    strategy_context: StrategyContext<()>,
}

#[async_trait]
impl StrategyTrait<()> for PrintLivePriceStrategy {
    async fn new(strategy_context: StrategyContext<()>) -> Self {
        PrintLivePriceStrategy { strategy_context }
    }

    async fn start(&self) {
        let subscription = &self.strategy_context.broker_subscription_list[0];
        let (mut receiver, _) = subscription
            .real_time_info(QueryInfoRequest {
                symbol: Symbol {
                    market: Market::US,
                    identifier: "ABNB".to_owned(),
                },
                kind: QuoteKind::Stock,
            })
            .await
            .unwrap();
        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]").unwrap();

        loop {
            select! {
                result = receiver.recv() => {
                    match result {
                        Some(quote_info) => {
                            log::info!(
                                "[{}] ({}), Price: {}, Vol: {}",
                                quote_info.symbol.to_string(),
                                OffsetDateTime::from_unix_timestamp(quote_info.timestamp as i64)
                                    .unwrap()
                                    .format(&format)
                                    .unwrap(),
                                quote_info.current_price.to_string(),
                                quote_info.volume,
                            );
                        },
                        None => {
                            log::error!("Received empty data from socket subscription, program will exit.");
                            return;
                        }
                    }
                }
            };
        }
    }

    async fn stop(&self) {
        // todo: provide an approach to stop gracefully
    }
}
