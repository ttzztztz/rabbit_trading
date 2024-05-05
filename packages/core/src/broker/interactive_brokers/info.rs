use anyhow::Error;
use async_trait::async_trait;
use ibkr_client_portal::{
    client::IBClientPortal,
    model::{
        contract::GetContractDetailRequest, definition::TickType, market_data::GetMarketDataRequest,
    },
};
use rust_decimal_macros::dec;

use super::broker::InteractiveBrokersBroker;
use crate::{
    broker::common::info::InfoTrait,
    model::{
        common::types::ConfigMap,
        trading::quote::{
            Depth, QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo,
        },
    },
    utils::time::get_now_unix_timestamp,
};

pub struct InteractiveBrokersInfo {
    client_portal: IBClientPortal,
}

#[async_trait]
impl InfoTrait for InteractiveBrokersInfo {
    fn new(config_map: ConfigMap) -> Self {
        let client_portal = InteractiveBrokersBroker::create_ib_client_portal(config_map);
        InteractiveBrokersInfo { client_portal }
    }

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        let conid = InteractiveBrokersBroker::get_conid_from_symbol(&request.symbol).await;
        let response = self
            .client_portal
            .get_contract_detail(GetContractDetailRequest { conid })
            .await?;

        Result::Ok(QuoteBasicInfo {
            symbol: request.symbol.clone(), // TODO
            currency: Option::Some(InteractiveBrokersBroker::to_currency(&response.currency)?),
            lot_size: 0,                 // TODO
            total_shares: dec!(0),       // TODO
            circulating_shares: dec!(0), // TODO
            eps: dec!(0),                // TODO
            eps_ttm: dec!(0),            // TODO
            bps: dec!(0),                // TODO
            dividend_yield: dec!(0),     // TODO
        })
    }

    async fn query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error> {
        let conid = InteractiveBrokersBroker::get_conid_from_symbol(&request.symbol).await;
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
                    TickType::Volume,
                    TickType::PriorClose,
                ]),
            })
            .await?;
        let result = response.first().unwrap(); // TODO: eliminate this unwrap()

        let timestamp = get_now_unix_timestamp();
        Result::Ok(QuoteRealTimeInfo {
            symbol: request.symbol.clone(),
            sequence: timestamp,
            timestamp,
            current_price: serde_json::from_value(
                result[TickType::LastPrice.to_string().as_str()].clone(),
            )
            .unwrap(), // TODO: eliminate this unwrap()
            volume: serde_json::from_value(result[TickType::Volume.to_string().as_str()].clone())
                .unwrap(), // TODO: eliminate this unwrap()
            low_price: serde_json::from_value(result[TickType::Low.to_string().as_str()].clone())
                .unwrap(), // TODO: eliminate this unwrap()
            high_price: serde_json::from_value(result[TickType::High.to_string().as_str()].clone())
                .unwrap(), // TODO: eliminate this unwrap()
            open_price: serde_json::from_value(result[TickType::Open.to_string().as_str()].clone())
                .unwrap(), // TODO: eliminate this unwrap()
            prev_close: serde_json::from_value(
                result[TickType::PriorClose.to_string().as_str()].clone(),
            )
            .unwrap(), // TODO: eliminate this unwrap()
            turnover: Option::None, // TODO: eliminate this unwrap()
            extra: Option::None,    // TODO: eliminate this unwrap()
        })
    }

    async fn query_depth(&self, request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error> {
        log::warn!("IBKR only supports 1 level depth data at this time"); // TODO: supports more

        let conid = InteractiveBrokersBroker::get_conid_from_symbol(&request.symbol).await;
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
        let result = response.first().unwrap(); // TODO: eliminate this unwrap()
        let ask_depth = Depth {
            position: Option::None,
            price: serde_json::from_value(result[TickType::AskPrice.to_string().as_str()].clone())
                .unwrap(),
            volume: serde_json::from_value(result[TickType::AskSize.to_string().as_str()].clone())
                .unwrap(),
            order_count: Option::None,
        };
        let bid_depth = Depth {
            position: Option::None,
            price: serde_json::from_value(result[TickType::BidPrice.to_string().as_str()].clone())
                .unwrap(),
            volume: serde_json::from_value(result[TickType::BidSize.to_string().as_str()].clone())
                .unwrap(),
            order_count: Option::None,
        };

        let timestamp = get_now_unix_timestamp();
        Result::Ok(QuoteDepthInfo {
            symbol: request.symbol.clone(),
            sequence: timestamp,
            timestamp,
            ask_list: vec![ask_depth],
            bid_list: vec![bid_depth],
        })
    }
}
