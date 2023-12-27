use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

use super::{
    broker::LongBridgeBroker,
    worker::{
        depth_info::{
            LongBridgeQuoteDepthInfoSubscriptionController,
            LongBridgeQuoteDepthInfoSubscriptionWorker,
        },
        real_time_info::{
            LongBridgeQuoteRealTimeInfoSubscriptionController,
            LongBridgeQuoteRealTimeInfoSubscriptionWorker,
        },
    },
};
use crate::{
    broker::common::subscription::{SubscriptionData, SubscriptionTrait, SubscriptionWorker},
    model::{
        error::Error,
        quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
    },
};

// https://crates.io/crates/longbridge
pub(super) struct LongBridgeSubscription {}

impl LongBridgeSubscription {}

#[async_trait]
impl SubscriptionTrait for LongBridgeSubscription {
    async fn new() -> Self {
        LongBridgeSubscription {}
    }

    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        let (longbridge_context, longbridge_receiver) =
            LongBridgeBroker::create_quote_context().await.unwrap();
        let (sys_sender, sys_receiver) = mpsc::channel(64);
        let longbridge_context_ref = Arc::new(Mutex::new(longbridge_context));

        let worker = LongBridgeQuoteRealTimeInfoSubscriptionWorker::new(
            request.symbol.clone(),
            sys_sender,
            longbridge_context_ref.clone(),
            longbridge_receiver,
        );
        let controller = LongBridgeQuoteRealTimeInfoSubscriptionController::new(
            request.symbol.clone(),
            longbridge_context_ref.clone(),
        );
        tokio::task::spawn(worker.start());
        Result::Ok((sys_receiver, Box::new(controller)))
    }

    async fn depth_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        let (longbridge_context, longbridge_receiver) =
            LongBridgeBroker::create_quote_context().await.unwrap();
        let (sys_sender, sys_receiver) = mpsc::channel(64);
        let longbridge_context_ref = Arc::new(Mutex::new(longbridge_context));

        let worker = LongBridgeQuoteDepthInfoSubscriptionWorker::new(
            request.symbol.clone(),
            sys_sender,
            longbridge_context_ref.clone(),
            longbridge_receiver,
        );
        let controller = LongBridgeQuoteDepthInfoSubscriptionController::new(
            request.symbol.clone(),
            longbridge_context_ref.clone(),
        );
        tokio::task::spawn(worker.start());
        Result::Ok((sys_receiver, Box::new(controller)))
    }
}

#[cfg(test)]
mod test_longbridge_subscription {
    use log;
    use rust_decimal_macros::dec;
    use tokio::time::{sleep, Duration};

    use super::LongBridgeSubscription;
    use crate::{
        broker::common::subscription::SubscriptionTrait,
        model::{
            market::Market,
            quote::{QueryInfoRequest, QuoteKind},
            symbol::Symbol,
        },
    };

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_quote_real_time_info() {
        let longbridge_subscription = LongBridgeSubscription::new().await;
        let (mut receiver, _) = longbridge_subscription
            .real_time_info(QueryInfoRequest {
                symbol: Symbol {
                    market: Market::HK,
                    identifier: "0700".to_owned(),
                },
                kind: QuoteKind::Stock,
            })
            .await
            .unwrap();
        tokio::select! {
            quote_info = receiver.recv() => {
                assert!(quote_info.is_some());
                let quote_info = quote_info.unwrap();
                log::warn!("quote_info: {quote_info:?}");
                assert_eq!("0700.HK", quote_info.symbol.to_string());
                assert!(quote_info.current_price > dec!(0.0));
                assert!(quote_info.volume > 0u64);
                assert!(quote_info.timestamp > 0u64);
            },
            _ = sleep(Duration::from_millis(3000))=> {
                panic!("loop not working!");
            },
        };
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_quote_depth_info() {
        let longbridge_subscription = LongBridgeSubscription::new().await;
        let (mut receiver, _) = longbridge_subscription
            .depth_info(QueryInfoRequest {
                symbol: Symbol {
                    market: Market::US,
                    identifier: "AAPL".to_owned(),
                },
                kind: QuoteKind::Stock,
            })
            .await
            .unwrap();
        tokio::select! {
            quote_depth_info_result = receiver.recv() => {
                assert!(quote_depth_info_result.is_some());
                let quote_depth_info = quote_depth_info_result.unwrap();
                log::warn!("quote_depth_info: {quote_depth_info:?}");
                assert_eq!("AAPL.US", quote_depth_info.symbol.to_string());
                assert!(quote_depth_info.ask_list.len() > 0);
                assert!(quote_depth_info.bid_list.len() > 0);
                quote_depth_info
                    .ask_list
                    .into_iter()
                    .chain(quote_depth_info.bid_list.into_iter())
                    .for_each(|depth| {
                        assert!(depth.order_count >= 0i64);
                        assert!(depth.position > 0i32);
                        assert!(depth.price > dec!(0.0));
                        assert!(depth.volume > 0i64);
                    });
            },
            _ = sleep(Duration::from_millis(3000))=> {
                panic!("loop not working!");
            },
        };
    }
}
