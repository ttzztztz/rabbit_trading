use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use super::info::YahooFinanceInfo;
use crate::broker::common::subscription::{
    SubscriptionController, SubscriptionData, SubscriptionWorker,
};
use crate::broker::common::{info::InfoTrait, subscription::SubscriptionTrait};
use crate::model::error::Error;
use crate::model::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo};

pub struct YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
    request: QueryInfoRequest,
    sender: Sender<QuoteRealTimeInfo>,

    working_flag: Arc<Mutex<bool>>,
}

impl YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
    pub fn new(
        request: QueryInfoRequest,
        sender: Sender<QuoteRealTimeInfo>,
        working_flag: Arc<Mutex<bool>>,
    ) -> Self {
        YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
            request,
            sender,
            working_flag,
        }
    }
}

#[async_trait]
impl SubscriptionWorker for YahooFinanceQuoteRealTimeInfoSubscriptionWorker {
    async fn start(self) {
        let info = YahooFinanceInfo::new().await;

        loop {
            if *self.working_flag.lock().await == false {
                return;
            }

            let real_time_info_result = info.query_real_time_info(self.request.clone()).await;
            if let Result::Ok(quote_info) = real_time_info_result {
                if let Err(send_result_err) = self.sender.send(quote_info).await {
                    log::error!("error when sending into mpsc {}", send_result_err);
                }
            }
            sleep(Duration::from_millis(1000)).await;
        }
    }
}

pub struct YahooFinanceQuoteRealTimeInfoSubscriptionController {
    working_flag: Arc<Mutex<bool>>,
}

impl YahooFinanceQuoteRealTimeInfoSubscriptionController {
    pub fn new(working_flag: Arc<Mutex<bool>>) -> Self {
        YahooFinanceQuoteRealTimeInfoSubscriptionController { working_flag }
    }
}

#[async_trait]
impl SubscriptionController for YahooFinanceQuoteRealTimeInfoSubscriptionController {
    async fn stop(self) -> Result<(), Error> {
        *self.working_flag.lock().await = false;
        Result::Ok(())
    }
}

pub struct YahooFinanceSubscription {}

#[async_trait]
impl SubscriptionTrait for YahooFinanceSubscription {
    async fn new() -> Self {
        YahooFinanceSubscription {}
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
            market::Market,
            quote::{QueryInfoRequest, QuoteKind},
            symbol::Symbol,
        },
    };

    #[tokio::test]
    async fn test_subscribe_quote_real_time_info() {
        let yahoo_finance_subscription = YahooFinanceSubscription::new().await;
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
            _ = sleep(Duration::from_millis(3000))=> {
                panic!("loop not working!");
            },
        };
    }
}
