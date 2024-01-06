use async_trait::async_trait;

use super::{broker::InteractiveBrokersBroker, client_portal::client::IBClientPortal};
use crate::{
    broker::common::info::InfoTrait,
    model::{
        common::error::Error,
        trading::quote::{QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo},
    },
};

pub struct InteractiveBrokersInfo {
    client_portal: IBClientPortal,
}

#[async_trait]
impl InfoTrait for InteractiveBrokersBroker {
    async fn new() -> Self {
        todo!()
    }

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        todo!()
    }

    async fn query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error> {
        todo!()
    }

    async fn query_depth(&self, request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error> {
        todo!()
    }
}
