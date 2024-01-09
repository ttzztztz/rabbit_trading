use async_trait::async_trait;

use super::client_portal::client::IBClientPortal;
use crate::{
    broker::common::subscription::{SubscriptionData, SubscriptionTrait},
    model::{
        common::error::Error,
        trading::quote::{QueryInfoRequest, QuoteDepthInfo, QuoteRealTimeInfo},
    },
};

pub struct InteractiveBrokersSubscription {
    client_portal: IBClientPortal,
}

#[async_trait]
impl SubscriptionTrait for InteractiveBrokersSubscription {
    async fn new() -> Self {
        todo!()
    }

    async fn real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteRealTimeInfo>, Error> {
        todo!()
    }

    async fn depth_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<SubscriptionData<QuoteDepthInfo>, Error> {
        todo!()
    }
}