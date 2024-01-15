use async_trait::async_trait;
use ibkr_client_portal::client::IBClientPortal;

use super::broker::InteractiveBrokersBroker;
use crate::{
    broker::common::info::InfoTrait,
    model::{
        common::{error::Error, types::ConfigMap},
        trading::quote::{QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo},
    },
};

pub struct InteractiveBrokersInfo {
    client_portal: IBClientPortal,
}

#[async_trait]
impl InfoTrait for InteractiveBrokersInfo {
    async fn new(config_map: ConfigMap) -> Self {
        let client_portal = InteractiveBrokersBroker::create_ib_client_portal(config_map);
        InteractiveBrokersInfo { client_portal }
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