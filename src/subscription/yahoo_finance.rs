use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use super::subscription_trait::Subscription;
use crate::info::info_trait::{Info, InfoContext};
use crate::info::yahoo_finance::YahooFinanceInfo;
use crate::model::error::Error;
use crate::model::quote::QuoteInfo;

struct YahooFinanceSubscription {
    info: Arc<Mutex<YahooFinanceInfo>>,
    stop_flag: Arc<Mutex<bool>>,
}

impl YahooFinanceSubscription {
    async fn start_loop(&self, sender: Sender<QuoteInfo>) {
        loop {
            if *self.stop_flag.lock().await == true {
                return;
            }

            if let Result::Ok(quote_info) = (*self.info.lock().await).query_real_time_info().await {
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
        let info = Arc::new(Mutex::new(YahooFinanceInfo::new(context).await));
        YahooFinanceSubscription {
            info,
            stop_flag: Arc::new(Mutex::new(false)),
        }
    }

    async fn subscribe(&self) -> Result<Receiver<QuoteInfo>, Error> {
        let (sender, receiver) = mpsc::channel(64);
        self.start_loop(sender); // todo: handle multi threading issue
        Result::Ok(receiver)
    }

    async fn unsubscribe(&self) -> Result<(), Error> {
        *self.stop_flag.lock().await = false;
        Result::Ok(())
    }
}
