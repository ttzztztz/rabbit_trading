use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use super::info::YahooFinanceInfo;
use crate::broker::common::{
    info_trait::{Info, InfoContext},
    subscription_trait::Subscription,
};
use crate::model::error::Error;
use crate::model::quote::QuoteInfo;

pub(super) struct YahooFinanceSubscription {
    context: InfoContext,
    stop_flag: Arc<Mutex<bool>>,
}

impl YahooFinanceSubscription {
    async fn start_loop(
        context: InfoContext,
        stop_flag: Arc<Mutex<bool>>,
        sender: Sender<QuoteInfo>,
    ) {
        let info = YahooFinanceInfo::new(context).await;

        loop {
            if *stop_flag.lock().await == true {
                return;
            }

            let real_time_info_result = info.query_real_time_info().await;
            if let Result::Ok(quote_info) = real_time_info_result {
                if let Err(send_result_err) = sender.send(quote_info).await {
                    log::error!("error when sending into mpsc {}", send_result_err);
                }
            }
            sleep(Duration::from_millis(500)).await;
        }
    }
}

#[async_trait]
impl Subscription for YahooFinanceSubscription {
    async fn new(context: InfoContext) -> Self {
        YahooFinanceSubscription {
            context,
            stop_flag: Arc::new(Mutex::new(false)),
        }
    }

    async fn subscribe(&self) -> Result<Receiver<QuoteInfo>, Error> {
        let (sender, receiver) = mpsc::channel(64);
        tokio::task::spawn(Self::start_loop(
            self.context.clone(),
            self.stop_flag.clone(),
            sender,
        ));
        Result::Ok(receiver)
    }

    async fn unsubscribe(&self) -> Result<(), Error> {
        *self.stop_flag.lock().await = false;
        Result::Ok(())
    }
}

#[cfg(test)]
mod test_yahoo_finance_subscription {
    use log;
    use rust_decimal_macros::dec;
    use tokio::time::{sleep, Duration};

    use super::YahooFinanceSubscription;
    use crate::{
        broker::common::{info_trait::InfoContext, subscription_trait::Subscription},
        model::quote::{Region, Symbol},
    };

    #[tokio::test]
    async fn test_query_quote_info() {
        let yahoo_finance_subscription = YahooFinanceSubscription::new(InfoContext {
            symbol: Symbol {
                identifier: "ABNB".to_owned(),
                region: Region::US,
            },
            extra: Option::None,
        })
        .await;

        let mut receiver = yahoo_finance_subscription.subscribe().await.unwrap();
        tokio::select! {
            quote_info = receiver.recv() => {
                assert!(quote_info.is_some());
                let quote_info = quote_info.unwrap();
                log::warn!("quote_info: {quote_info:?}");
                assert_eq!("ABNB.US", quote_info.symbol.to_string());
                assert!(quote_info.current_price > dec!(0.0));
                assert!(quote_info.volume > 0u64);
                assert!(quote_info.timestamp > 0i64);
            },
            _ = sleep(Duration::from_millis(3000))=> {
                panic!("loop not working!");
            },
        };
    }
}
