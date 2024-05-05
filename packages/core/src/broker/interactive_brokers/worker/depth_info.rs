use anyhow::{anyhow, Error};
use async_trait::async_trait;
use ibkr_client_portal::model::{
    definition::TickType,
    streaming::{StreamingDataResponse, SubscribeMarketDataRequest, ToStructuredRequest, UnsubscribeMarketDataRequest},
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::Sender;

use crate::{
    broker::{
        common::subscription::{SubscriptionController, SubscriptionWorker},
        interactive_brokers::broker::InteractiveBrokersBroker,
    },
    model::{
        common::types::ConfigMap,
        trading::{quote::QuoteDepthInfo, symbol::Symbol},
    },
};

pub struct IBQuoteDepthInfoSubscriptionWorker {
    config_map: ConfigMap,
    symbol: Symbol,
    sys_sender: Sender<QuoteDepthInfo>,
    local_stopped_indicator: Arc<AtomicBool>,
    global_stopped_indicator: Arc<AtomicBool>,
}

impl IBQuoteDepthInfoSubscriptionWorker {
    pub fn new(
        config_map: ConfigMap,
        symbol: Symbol,
        sys_sender: Sender<QuoteDepthInfo>,
        local_stopped_indicator: Arc<AtomicBool>,
        global_stopped_indicator: Arc<AtomicBool>,
    ) -> Self {
        IBQuoteDepthInfoSubscriptionWorker {
            config_map,
            symbol,
            sys_sender,
            local_stopped_indicator,
            global_stopped_indicator,
        }
    }
}

#[async_trait]
impl SubscriptionWorker for IBQuoteDepthInfoSubscriptionWorker {
    async fn start(mut self) -> Result<(), Error> {
        let conid = InteractiveBrokersBroker::get_conid_from_symbol(&self.symbol).await;
        let client_portal =
            InteractiveBrokersBroker::create_ib_client_portal(self.config_map.clone());
        let (sender, receiver) = client_portal.connect_to_websocket().await.unwrap();
        if let Err(err) = sender
            .send_streaming_structured_data_request(
                SubscribeMarketDataRequest {
                    conid: format!("{}", conid),
                    fields: vec![
                        TickType::AskPrice,
                        TickType::AskSize,
                        TickType::BidPrice,
                        TickType::BidSize,
                    ]
                    .into_iter()
                    .map(|field| field.to_string())
                    .collect(),
                }
                .to_structured_request(),
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
                        UnsubscribeMarketDataRequest {
                            conid: format!("{}", conid),
                        }
                        .to_structured_request(),
                    )
                    .await
                {
                    return Result::Err(anyhow!("Error when closing streaming {:?}", err));
                }
                return Result::Ok(());
            }

            match receiver.receive().await {
                Ok(streaming_data) => {
                    if let Err(send_err) = self
                        .sys_sender
                        .send(QuoteDepthInfo {
                            symbol: todo!(),
                            sequence: todo!(),
                            timestamp: todo!(),
                            ask_list: todo!(),
                            bid_list: todo!(),
                        })
                        .await
                    {
                        log::warn!("Error when sending message {:?}", send_err);
                    }
                }
                Err(streaming_err) => {
                    log::warn!("Streaming Error {:?}", streaming_err);
                }
            }
        }
    }
}

pub struct IBQuoteDepthInfoSubscriptionController {
    local_stopped_indicator: Arc<AtomicBool>,
}

impl IBQuoteDepthInfoSubscriptionController {
    pub fn new(local_stopped_indicator: Arc<AtomicBool>) -> Self {
        IBQuoteDepthInfoSubscriptionController {
            local_stopped_indicator,
        }
    }
}

#[async_trait]
impl SubscriptionController for IBQuoteDepthInfoSubscriptionController {
    async fn stop(self) -> Result<(), Error> {
        self.local_stopped_indicator.store(false, Ordering::Relaxed);
        Result::Ok(())
    }
}
