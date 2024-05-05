use anyhow::{anyhow, Error};
use async_trait::async_trait;
use ibkr_client_portal::model::streaming::{
    SubscribeMarketDataRequest, ToStructuredRequest, UnsubscribeMarketDataRequest,
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
        trading::{quote::QuoteRealTimeInfo, symbol::Symbol},
    },
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
                SubscribeMarketDataRequest {
                    conid: format!("{}", conid),
                    fields: vec![],
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
                        .send(QuoteRealTimeInfo {
                            symbol: todo!(),
                            sequence: todo!(),
                            timestamp: todo!(),
                            current_price: todo!(),
                            volume: todo!(),
                            low_price: todo!(),
                            high_price: todo!(),
                            open_price: todo!(),
                            prev_close: todo!(),
                            turnover: todo!(),
                            extra: todo!(),
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
