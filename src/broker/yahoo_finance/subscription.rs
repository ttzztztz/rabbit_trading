use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use super::worker::real_time_info::{
    YahooFinanceQuoteRealTimeInfoSubscriptionController,
    YahooFinanceQuoteRealTimeInfoSubscriptionWorker,
};
use crate::broker::common::{
    subscription::SubscriptionTrait,
    subscription::{SubscriptionData, SubscriptionWorker},
};
use crate::model::{
    common::{error::Error, types::ConfigMap},
    trading::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
};

pub struct YahooFinanceSubscription {
    config_map: ConfigMap,
}

#[async_trait]
impl SubscriptionTrait for YahooFinanceSubscription {
    async fn new(config_map: ConfigMap) -> Self {
        YahooFinanceSubscription { config_map }
    }

    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        let (sender, receiver) = mpsc::channel(64);

        let working_flag = Arc::new(Mutex::new(true));
        let worker = YahooFinanceQuoteRealTimeInfoSubscriptionWorker::new(
            request,
            sender,
            working_flag.clone(),
        );
        let controller =
            YahooFinanceQuoteRealTimeInfoSubscriptionController::new(working_flag.clone());

        tokio::task::spawn(worker.start());
        Result::Ok((receiver, Box::new(controller)))
    }

    async fn depth_info(
        &self,
        _request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        todo!()
    }
}

#[cfg(test)]
mod test_yahoo_finance_subscription {
    use log;
    use rust_decimal_macros::dec;
    use tokio::time::{sleep, Duration};

    use super::YahooFinanceSubscription;
    use crate::{
        broker::common::subscription::SubscriptionTrait,
        model::{
            common::types::ConfigMap,
            trading::{
                market::Market,
                quote::{QueryInfoRequest, QuoteKind},
                symbol::Symbol,
            },
        },
    };

    #[tokio::test]
    async fn test_subscribe_quote_real_time_info() {
        let yahoo_finance_subscription = YahooFinanceSubscription::new(ConfigMap::new()).await;
        let subscription_instance_result = yahoo_finance_subscription
            .real_time_info(QueryInfoRequest {
                symbol: Symbol {
                    market: Market::US,
                    identifier: "ABNB".to_owned(),
                },
                kind: QuoteKind::Stock,
            })
            .await;
        assert!(subscription_instance_result.is_ok());
        let (mut receiver, _) = subscription_instance_result.unwrap();
        tokio::select! {
            quote_info = receiver.recv() => {
                assert!(quote_info.is_some());
                let quote_info = quote_info.unwrap();
                log::warn!("quote_info: {quote_info:?}");
                assert_eq!("ABNB.US", quote_info.symbol.to_string());
                assert!(quote_info.current_price > dec!(0.0));
                assert!(quote_info.volume > 0u64);
                assert!(quote_info.timestamp > 0u64);
            },
            _ = sleep(Duration::from_millis(5000))=> {
                panic!("loop not working!");
            },
        };
    }
}
