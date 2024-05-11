use anyhow::{anyhow, Error};
use async_trait::async_trait;
use ibkr_client_portal::model::{
    definition::TickType,
    streaming::{
        MarketDataResponse, StreamingDataResponse, StreamingDataStructuredRequest,
        SubscribeMarketDataRequest, ToStructuredRequest, UnsubscribeMarketDataRequest,
    },
};
use rust_decimal::Decimal;
use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::mpsc::Sender;

use crate::{
    broker::{
        common::subscription::{SubscriptionController, SubscriptionWorker},
        interactive_brokers::broker::InteractiveBrokersBroker,
    },
    model::{
        common::types::ConfigMap,
        trading::{quote::QuoteRealTimeInfo, symbol::Symbol},
    },
    utils::time::get_now_unix_timestamp,
};

pub struct IBQuoteRealTimeInfoSubscriptionWorker {
    config_map: ConfigMap,
    symbol: Symbol,
    sys_sender: Sender<QuoteRealTimeInfo>,
    local_stopped_indicator: Arc<AtomicBool>,
    global_stopped_indicator: Arc<AtomicBool>,
}

impl IBQuoteRealTimeInfoSubscriptionWorker {
    pub fn new(
        config_map: ConfigMap,
        symbol: Symbol,
        sys_sender: Sender<QuoteRealTimeInfo>,
        local_stopped_indicator: Arc<AtomicBool>,
        global_stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        IBQuoteRealTimeInfoSubscriptionWorker {
            config_map,
            symbol,
            sys_sender,
            local_stopped_indicator,
            global_stopped_indicator,
        }
    }

    fn create_subscribe_market_data_structured_request(
        conid: i64,
    ) -> StreamingDataStructuredRequest {
        SubscribeMarketDataRequest {
            conid: format!("{}", conid),
            fields: vec![
                TickType::LastPrice,
                TickType::High,
                TickType::Low,
                TickType::Open,
                TickType::Volume,
                TickType::PriorClose,
            ]
            .into_iter()
            .map(|field| field.to_string())
            .collect(),
        }
        .to_structured_request()
    }

    fn create_unsubscribe_market_data_structured_request(
        conid: i64,
    ) -> StreamingDataStructuredRequest {
        UnsubscribeMarketDataRequest {
            conid: format!("{}", conid),
        }
        .to_structured_request()
    }

    fn market_data_response_to_quote_real_time_info(
        symbol: Symbol,
        data: MarketDataResponse,
    ) -> QuoteRealTimeInfo {
        let timestamp = get_now_unix_timestamp();
        QuoteRealTimeInfo {
            symbol,
            sequence: timestamp,
            timestamp,
            // todo: Handle C and H prefix
            current_price: Decimal::from_str(data.last_price.clone().unwrap().as_str()).unwrap(), // TODO: eliminate this unwrap()
            volume: data.volume.clone().unwrap().parse().unwrap(), // TODO: eliminate this unwrap()
            low_price: data.low_price,                             // TODO: eliminate this unwrap()
            high_price: data.high_price,                           // TODO: eliminate this unwrap()
            open_price: data.open,                                 // TODO: eliminate this unwrap()
            prev_close: data.prior_close,                          // TODO: eliminate this unwrap()
            turnover: Option::None,                                // TODO: eliminate this unwrap()
            extra: Option::None,                                   // TODO: eliminate this unwrap()
        }
    }
}

#[async_trait]
impl SubscriptionWorker for IBQuoteRealTimeInfoSubscriptionWorker {
    async fn start(mut self) -> Result<(), Error> {
        let conid = InteractiveBrokersBroker::get_conid_from_symbol(&self.symbol).await;
        let client_portal =
            InteractiveBrokersBroker::create_ib_client_portal(self.config_map.clone());
        let (sender, receiver) = client_portal.connect_to_websocket().await.unwrap();
        if let Err(err) = sender
            .send_streaming_structured_data_request(
                Self::create_subscribe_market_data_structured_request(conid),
            )
            .await
        {
            return Result::Err(anyhow!("Error when subscribing market data {:?}", err));
        }

        loop {
            if self.global_stopped_indicator.load(Ordering::Relaxed)
                || self.local_stopped_indicator.load(Ordering::Relaxed)
            {
                if let Err(err) = sender
                    .send_streaming_structured_data_request(
                        Self::create_unsubscribe_market_data_structured_request(conid),
                    )
                    .await
                {
                    return Result::Err(anyhow!("Error when closing streaming {:?}", err));
                }
                return Result::Ok(());
            }

            match receiver.receive().await {
                Ok(streaming_data) => match streaming_data {
                    StreamingDataResponse::MarketData(data) => {
                        if let Err(send_err) = self
                            .sys_sender
                            .send(Self::market_data_response_to_quote_real_time_info(
                                self.symbol.clone(),
                                data,
                            ))
                            .await
                        {
                            log::warn!("Error when sending message {:?}", send_err);
                        }
                    }
                    _ => continue,
                },
                Err(streaming_err) => {
                    log::warn!("Streaming Error {:?}", streaming_err);
                }
            }
        }
    }
}

pub struct IBQuoteRealTimeInfoSubscriptionController {
    local_stopped_indicator: Arc<AtomicBool>,
}

impl IBQuoteRealTimeInfoSubscriptionController {
    pub fn new(local_stopped_indicator: Arc<AtomicBool>) -> Self {
        IBQuoteRealTimeInfoSubscriptionController {
            local_stopped_indicator,
        }
    }
}

#[async_trait]
impl SubscriptionController for IBQuoteRealTimeInfoSubscriptionController {
    async fn stop(self) -> Result<(), Error> {
        self.local_stopped_indicator.store(false, Ordering::Relaxed);
        Result::Ok(())
    }
}
