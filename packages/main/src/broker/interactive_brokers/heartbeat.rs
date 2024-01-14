use async_trait::async_trait;
use ibkr_client_portal::client::IBClientPortal;
use tokio::time::{sleep, Duration};

use super::broker::InteractiveBrokersBroker;
use crate::{
    broker::common::heartbeat::HeartbeatTrait,
    model::common::{error::Error, types::ConfigMap},
    utils::error::reqwest_error_to_rabbit_trading_error,
};

pub struct InteractiveBrokersHeartbeat {
    client_portal: IBClientPortal,
}

#[async_trait]
impl HeartbeatTrait for InteractiveBrokersHeartbeat {
    async fn new(config_map: ConfigMap) -> Self {
        InteractiveBrokersHeartbeat {
            client_portal: InteractiveBrokersBroker::create_ib_client_portal(config_map),
        }
    }

    async fn start(&self) -> Result<(), Error> {
        loop {
            if let Err(err) = self
                .client_portal
                .tickle()
                .await
                .map_err(reqwest_error_to_rabbit_trading_error)
            {
                log::error!("Error when tickle {}", err.message);
            }
            sleep(Duration::from_millis(1000)).await;
        }
    }

    async fn stop(&self) -> Result<(), Error> {
        todo!()
    }
}
