use async_trait::async_trait;
use time::{format_description, OffsetDateTime};

use crate::{
    broker::common::{info::InfoContext, subscription::SubscriptionTrait},
    model::{market::Market, symbol::Symbol},
    strategy::common::strategy::{StrategyContext, StrategyTrait},
};

pub struct PrintLivePriceStrategy {
    subscription: Box<dyn SubscriptionTrait + Send + Sync>,
}

#[async_trait]
impl StrategyTrait<()> for PrintLivePriceStrategy {
    async fn new(context: StrategyContext<()>) -> Self {
        let broker = &context.broker_list[0];
        let subscription = broker
            .create_subscription(InfoContext {
                symbol: Symbol {
                    market: Market::US,
                    identifier: "ABNB".to_owned(),
                },
                extra: Option::None,
            })
            .await;

        PrintLivePriceStrategy { subscription }
    }

    async fn start(&self) {
        let mut receiver = self.subscription.subscribe().await.unwrap();

        let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]").unwrap();
        while let Some(quote_info) = receiver.recv().await {
            log::info!(
                "[{}] ({}), Price: {}, Vol: {}",
                quote_info.symbol.to_string(),
                OffsetDateTime::from_unix_timestamp(quote_info.timestamp)
                    .unwrap()
                    .format(&format)
                    .unwrap(),
                quote_info.current_price.to_string(),
                quote_info.volume,
            );
        }
    }

    async fn stop(&self) {
        self.subscription.unsubscribe().await.unwrap();
    }
}
