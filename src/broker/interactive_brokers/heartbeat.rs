use async_trait::async_trait;
use tokio::time::{sleep, Duration};

use super::{broker::InteractiveBrokersBroker, client_portal::client::IBClientPortal};
use crate::{
    broker::common::heartbeat::HeartbeatTrait,
    model::common::{error::Error, types::ConfigMap},
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
            if let Err(err) = self.client_portal.tickle().await {
                log::error!("Error when tickle {}", err.message);
            }
            sleep(Duration::from_millis(1000)).await;
        }
    }

    async fn stop(&self) -> Result<(), Error> {
        todo!()
    }
}
