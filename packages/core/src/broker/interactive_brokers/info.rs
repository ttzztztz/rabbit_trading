use anyhow::Error;
use async_trait::async_trait;
use ibkr_client_portal::client::IBClientPortal;

use super::broker::InteractiveBrokersBroker;
use crate::{
    broker::common::info::InfoTrait,
    model::{
        common::types::ConfigMap,
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

    async fn query_basic_info(&self, _request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        todo!()
    }

    async fn query_real_time_info(
        &self,
        _request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error> {
        todo!()
    }

    async fn query_depth(&self, _request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error> {
        todo!()
    }
}
