use anyhow::Error;
use async_trait::async_trait;
use ibkr_client_portal::client::IBClientPortal;

use crate::{
    broker::common::subscription::{SubscriptionData, SubscriptionTrait},
    model::{
        common::types::ConfigMap,
        trading::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
    },
};

pub struct InteractiveBrokersSubscription {
    client_portal: IBClientPortal,
}

#[async_trait]
impl SubscriptionTrait for InteractiveBrokersSubscription {
    fn new(_config_map: ConfigMap) -> Self {
        todo!()
    }

    async fn real_time_info(
        &self,
        _request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        todo!()
    }

    async fn depth_info(
        &self,
        _request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        todo!()
    }
}
