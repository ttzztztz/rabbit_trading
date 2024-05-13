use anyhow::Error;
use async_trait::async_trait;
use ibkr_client_portal::{
    client::IBClientPortal,
    model::{
        contract::{ContractDetail, GetContractDetailRequest},
        definition::TickType,
        market_data::{GetMarketDataRequest, MarketData},
    },
};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::str::FromStr;

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
        symbol: Symbol,
        contract_detail: ContractDetail,
    ) -> Result<QuoteBasicInfo, Error> {
        Result::Ok(QuoteBasicInfo {
            symbol, // TODO
            currency: Option::Some(InteractiveBrokersBroker::to_currency(
                &contract_detail.currency,
            )?),
            lot_size: 0,                 // TODO
            total_shares: dec!(0),       // TODO
            circulating_shares: dec!(0), // TODO
            eps: dec!(0),                // TODO
            eps_ttm: dec!(0),            // TODO
            bps: dec!(0),                // TODO
            dividend_yield: dec!(0),     // TODO
        })
    }

    fn market_data_to_quote_real_time_info(
        symbol: Symbol,
        market_data: &MarketData,
    ) -> QuoteRealTimeInfo {
        let timestamp = get_now_unix_timestamp();
        QuoteRealTimeInfo {
            symbol,
            sequence: timestamp,
            timestamp,
            // todo: Handle C and H prefix
            current_price: Decimal::from_str(market_data.last_price.clone().unwrap().as_str())
                .unwrap(), // TODO: eliminate this unwrap()
            volume: market_data.volume_long.unwrap(), // TODO: eliminate this unwrap()
            low_price: market_data.low_price,         // TODO: eliminate this unwrap()
            high_price: market_data.high_price,       // TODO: eliminate this unwrap()
            open_price: market_data.open,             // TODO: eliminate this unwrap()
            prev_close: market_data.prior_close,      // TODO: eliminate this unwrap()
            turnover: Option::None,                   // TODO: eliminate this unwrap()
            extra: Option::None,                      // TODO: eliminate this unwrap()
        }
    }

    fn market_data_to_quote_depth_info(symbol: Symbol, market_data: &MarketData) -> QuoteDepthInfo {
        let ask_depth = Depth {
            position: Option::None,
            price: market_data.ask_price.unwrap(),
            volume: market_data
                .ask_size
                .clone()
                .unwrap()
                .replace(",", "")
                .parse()
                .unwrap(), // TODO: handle the logics here
            order_count: Option::None,
        };
        let bid_depth = Depth {
            position: Option::None,
            price: market_data.bid_price.unwrap(),
            volume: market_data
                .bid_size
                .clone()
                .unwrap()
                .replace(",", "")
                .parse()
                .unwrap(), // TODO: handle the logics here
            order_count: Option::None,
        };

        let timestamp = get_now_unix_timestamp();
        QuoteDepthInfo {
            symbol,
            sequence: timestamp,
            timestamp,
            ask_list: vec![ask_depth],
            bid_list: vec![bid_depth],
        }
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
        let market_data = response.first().unwrap(); // TODO: eliminate this unwrap()
        Result::Ok(Self::market_data_to_quote_real_time_info(
            request.symbol.clone(),
            market_data,
        ))
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
        let market_data = response.first().unwrap(); // TODO: eliminate this unwrap()
        Result::Ok(Self::market_data_to_quote_depth_info(
            request.symbol.clone(),
            market_data,
        ))
    }
}
