use anyhow::{anyhow, Context, Error};
use async_trait::async_trait;
use ibkr_client_portal::{
    client::IBClientPortal,
    model::{
        contract::{ContractDetail, GetContractDetailRequest},
        definition::TickType,
        market_data::{GetMarketDataRequest, MarketData},
    },
};

use super::{broker::InteractiveBrokersBroker, config::IBConfig, symbol::IBSymbolHelper};
use crate::{
    broker::common::info::InfoTrait,
    model::{
        common::types::ConfigMap,
        trading::{
            quote::{Depth, QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo},
            symbol::Symbol,
        },
    },
    utils::time::get_now_unix_timestamp,
};

pub struct InteractiveBrokersInfo {
    client_portal: IBClientPortal,
    ib_symbol_helper: IBSymbolHelper,
}

impl InteractiveBrokersInfo {
    fn ib_contract_detail_to_quote_basic_info(
        _symbol: Symbol,
        _contract_detail: ContractDetail,
    ) -> Result<QuoteBasicInfo, Error> {
        todo!()
    }

    fn market_data_to_quote_real_time_info(
        symbol: Symbol,
        market_data: &MarketData,
    ) -> Result<QuoteRealTimeInfo, Error> {
        let sequence = get_now_unix_timestamp();
        let timestamp = market_data
            .updated
            .map(|val| val as u64)
            .unwrap_or(sequence);

        Result::Ok(QuoteRealTimeInfo {
            symbol,
            sequence,
            timestamp,
            current_price: InteractiveBrokersBroker::parse_last_price(
                market_data.last_price.clone(),
            )?,
            volume: market_data.volume_long,
            low_price: market_data.low_price,
            high_price: market_data.high_price,
            open_price: market_data.open,
            prev_close: market_data.prior_close,
            turnover: Option::None, // TODO: fill in this field
            extra: Option::None,
        })
    }

    fn market_data_to_quote_depth_info(
        symbol: Symbol,
        market_data: &MarketData,
    ) -> Result<QuoteDepthInfo, Error> {
        let ask_price = market_data
            .ask_price
            .with_context(|| format!("Error when retrieving ask_price for symbol {:?}", symbol))?;
        let bid_price = market_data
            .bid_price
            .with_context(|| format!("Error when retrieving bid_price for symbol {:?}", symbol))?;

        let ask_depth = Depth {
            position: Option::None,
            price: ask_price,
            volume: InteractiveBrokersBroker::depth_size_to_volume(market_data.ask_size.clone())
                .ok(),
            order_count: Option::None,
        };
        let bid_depth = Depth {
            position: Option::None,
            price: bid_price,
            volume: InteractiveBrokersBroker::depth_size_to_volume(market_data.bid_size.clone())
                .ok(),
            order_count: Option::None,
        };

        let sequence = get_now_unix_timestamp();
        let timestamp = market_data
            .updated
            .map(|val| val as u64)
            .unwrap_or(sequence);
        Result::Ok(QuoteDepthInfo {
            symbol,
            sequence,
            timestamp,
            ask_list: vec![ask_depth],
            bid_list: vec![bid_depth],
        })
    }
}

#[async_trait]
impl InfoTrait for InteractiveBrokersInfo {
    fn new(config_map: ConfigMap) -> Self {
        let client_portal = InteractiveBrokersBroker::create_ib_client_portal(config_map.clone());
        let ib_config = IBConfig::new(&config_map).unwrap();
        let ib_symbol_helper = IBSymbolHelper::new(ib_config);

        InteractiveBrokersInfo {
            client_portal,
            ib_symbol_helper,
        }
    }

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        let conid = self.ib_symbol_helper.get_conid(&request.symbol).unwrap();
        let contract_detail = self
            .client_portal
            .get_contract_detail(GetContractDetailRequest { conid })
            .await?;

        Result::Ok(Self::ib_contract_detail_to_quote_basic_info(
            request.symbol.clone(),
            contract_detail,
        )?)
    }

    async fn query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error> {
        let conid = self.ib_symbol_helper.get_conid(&request.symbol).unwrap();
        let response = self
            .client_portal
            .get_market_data(GetMarketDataRequest {
                conid_list: vec![conid],
                since: Option::None,
                fields: Option::Some(vec![
                    TickType::LastPrice,
                    TickType::High,
                    TickType::Low,
                    TickType::Open,
                    TickType::VolumeLong,
                    TickType::PriorClose,
                ]),
            })
            .await?;

        match response.first() {
            Some(market_data) => Result::Ok(Self::market_data_to_quote_real_time_info(
                request.symbol.clone(),
                market_data,
            )?),
            None => Result::Err(anyhow!(
                "Error when retrieving the real_time_info of the security {:?}",
                request
            )),
        }
    }

    async fn query_depth(&self, request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error> {
        log::warn!("IBKR only supports 1 level depth data at this time"); // TODO: supports more

        let conid = self.ib_symbol_helper.get_conid(&request.symbol).unwrap();
        let response = self
            .client_portal
            .get_market_data(GetMarketDataRequest {
                conid_list: vec![conid],
                since: Option::None,
                fields: Option::Some(vec![
                    TickType::AskPrice,
                    TickType::AskSize,
                    TickType::BidPrice,
                    TickType::BidSize,
                ]),
            })
            .await?;

        match response.first() {
            Some(market_data) => Result::Ok(Self::market_data_to_quote_depth_info(
                request.symbol.clone(),
                market_data,
            )?),
            None => Result::Err(anyhow!(
                "Error when retrieving the depth of the security {:?}",
                request
            )),
        }
    }
}
