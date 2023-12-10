use rust_decimal::Decimal;
use std::cell::RefCell;
use tokio::time::{sleep, Duration};
use yahoo_finance_api::YahooConnector;

use crate::info::info_trait::{InfoContext, InfoWorker};
use crate::model::quote::QuoteInfo;

struct YahooFinanceInfo {
    provider: YahooConnector,
    context: InfoContext,
    stop_flag: RefCell<bool>,
}

impl YahooFinanceInfo {
    const YAHOO_LAST_QUOTES_INTERVAL: &'static str = "1d";

    async fn query_quote_info(&self) -> Option<QuoteInfo> {
        let quote = &self.context.quote;
        match self
            .provider
            .get_latest_quotes(quote.identifier.as_str(), Self::YAHOO_LAST_QUOTES_INTERVAL)
            .await
            .and_then(|y_response| y_response.last_quote())
        {
            std::result::Result::Ok(yahoo_quote) => {
                log::info!("Received yahoo_quote = {yahoo_quote:?} successfully");

                Option::Some(QuoteInfo {
                    quote: quote.clone(),
                    sequence: yahoo_quote.timestamp,
                    timestamp: yahoo_quote.timestamp,
                    current_price: Decimal::from_str_exact(
                        format!("{:.2}", yahoo_quote.close).as_str(),
                    )
                    .unwrap(),
                    low_price: Option::None,
                    high_price: Option::None,
                    open_price: Option::None,
                    volume: yahoo_quote.volume,
                    turnover: Option::None,
                    extra: Option::None,
                })
            }
            std::result::Result::Err(err) => {
                log::error!("error {}", err);
                Option::None
            }
        }
    }

    async fn start_loop(&self) {
        loop {
            if self.stop_flag.take() == true {
                return;
            }

            if let Some(quote_info) = self.query_quote_info().await {
                if let Err(send_result_err) = self.context.sender.send(quote_info).await {
                    log::error!("error when sending into mpsc {}", send_result_err);
                }
            }
            sleep(Duration::from_millis(500)).await;
        }
    }
}

impl InfoWorker for YahooFinanceInfo {
    fn new(context: InfoContext) -> Self {
        let provider = YahooConnector::new();
        YahooFinanceInfo {
            provider,
            context,
            stop_flag: false.into(),
        }
    }

    fn start(&self) {
        self.context.runtime.block_on(self.start_loop());
    }

    fn stop(&self) {
        *self.stop_flag.borrow_mut() = true.into();
    }
}

#[cfg(test)]
mod test_yahoo_finance_info {
    use log::{self, LevelFilter};
    use longbridge::decimal;
    use simple_logger::SimpleLogger;
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    use tokio::sync::mpsc;

    use super::YahooFinanceInfo;
    use crate::info::info_trait::{InfoContext, InfoWorker};
    use crate::model::quote::Quote;

    #[test]
    fn test_query_quote_info() {
        SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init()
            .unwrap();

        let runtime = Arc::new(Runtime::new().unwrap());
        let (sender, _) = mpsc::channel(64);
        let sender_arc = Arc::new(sender);
        let yahoo_finance_info = YahooFinanceInfo::new(InfoContext {
            quote: Quote {
                kind: crate::model::quote::QuoteKind::Stock,
                identifier: "ABNB".to_owned(),
            },
            runtime: runtime.clone(),
            sender: sender_arc,
            extra: Option::None,
        });

        let quote_info_optional = runtime.block_on(yahoo_finance_info.query_quote_info());
        log::warn!("quote_info: {quote_info_optional:?}");
        assert!(quote_info_optional.is_some());
        let quote_info = quote_info_optional.unwrap();
        assert_eq!("Stock:ABNB", quote_info.quote.to_string());
        assert!(quote_info.current_price > decimal!(0.0));
        assert!(quote_info.volume > 0);
        assert!(quote_info.timestamp > 0);
    }
}
