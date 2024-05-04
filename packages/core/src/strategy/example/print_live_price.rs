use anyhow::{anyhow, Error};
use async_trait::async_trait;
use std::sync::atomic::Ordering;
use time::{format_description, OffsetDateTime};
use tokio::select;

use crate::{
    model::trading::{
        market::Market,
        quote::{QueryInfoRequest, QuoteKind},
        symbol::Symbol,
    },
    strategy::common::strategy::{StrategyContext, StrategyTrait},
};

pub struct PrintLivePriceStrategy {
    strategy_context: StrategyContext,
}

#[async_trait]
impl StrategyTrait for PrintLivePriceStrategy {
    fn new(strategy_context: StrategyContext) -> Self {
        PrintLivePriceStrategy { strategy_context }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "ExamplePrintLivePriceStrategy";
        IDENTIFIER.to_owned()
    }

    async fn start(&self) -> Result<(), Error> {
        let broker = &self.strategy_context.broker_list[0];
        let subscription = broker.create_subscription();

        let (mut receiver, _) = subscription
            .real_time_info(QueryInfoRequest {
                symbol: Symbol {
                    market: Market::US,
                    identifier: "ABNB".to_owned(),
                },
                kind: QuoteKind::Stock,
            })
            .await?;
        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]").unwrap();

        loop {
            if self
                .strategy_context
                .stopped_indicator
                .load(Ordering::Relaxed)
            {
                return Result::Ok(());
            }

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
                            return Result::Err(anyhow!("EMPTY_MESSAGE_RECEIVED, Received empty data from socket subscription, program will exit"));
                        }
                    }
                }
            };
        }
    }

    async fn stop(&self) -> Result<(), Error> {
        self.strategy_context
            .stopped_indicator
            .store(true, Ordering::Relaxed);
        Result::Ok(())
    }
}
