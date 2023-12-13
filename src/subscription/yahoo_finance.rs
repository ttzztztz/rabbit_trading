use std::cell::RefCell;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::{sleep, Duration};

use super::subscription_trait::Subscription;
use crate::info::info_trait::{Info, InfoContext};
use crate::info::yahoo_finance::YahooFinanceInfo;
use crate::model::error::Error;
use crate::model::quote::QuoteInfo;

struct YahooFinanceSubscription {
    info: YahooFinanceInfo,
    stop_flag: RefCell<bool>,
}

impl YahooFinanceSubscription {
    async fn start_loop(&self, sender: Sender<QuoteInfo>) {
        loop {
            if self.stop_flag.take() == true {
                return;
            }

            if let Result::Ok(quote_info) = self.info.query_quote_info().await {
                if let Err(send_result_err) = sender.send(quote_info).await {
                    log::error!("error when sending into mpsc {}", send_result_err);
                }
            }
            sleep(Duration::from_millis(500)).await;
        }
    }
}

impl Subscription for YahooFinanceSubscription {
    fn new(context: InfoContext) -> Self {
        let info = YahooFinanceInfo::new(context);
        YahooFinanceSubscription {
            info,
            stop_flag: false.into(),
        }
    }

    fn subscribe(&self) -> Result<Receiver<QuoteInfo>, Error> {
        let (sender, receiver) = mpsc::channel(64);
        self.start_loop(sender); // todo: handle multi threading issue
        Result::Ok(receiver)
    }

    fn unsubscribe(&self) -> Result<(), Error> {
        *self.stop_flag.borrow_mut() = false;
        Result::Ok(())
    }
}
