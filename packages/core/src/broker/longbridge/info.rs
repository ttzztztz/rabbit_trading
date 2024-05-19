use anyhow::{anyhow, Context, Error};
use async_trait::async_trait;
use longbridge::quote::{SecurityDepth, SecurityQuote, SecurityStaticInfo};
use std::result::Result;

use super::broker::LongBridgeBroker;
use crate::broker::common::info::InfoTrait;
use crate::model::{
    common::types::ConfigMap,
    trading::{
        quote::{QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo},
        symbol::Symbol,
    },
};
use crate::utils::time::get_now_unix_timestamp;

pub struct LongBridgeInfo {}

impl LongBridgeInfo {
    fn to_quote_real_time_info(symbol: Symbol, security_quote: SecurityQuote) -> QuoteRealTimeInfo {
        let unix_timestamp = security_quote.timestamp.unix_timestamp() as u64;
        QuoteRealTimeInfo {
            symbol,
            sequence: unix_timestamp,
            timestamp: unix_timestamp,
            current_price: security_quote.last_done,
            low_price: Option::Some(security_quote.low),
            high_price: Option::Some(security_quote.high),
            open_price: Option::Some(security_quote.open),
            prev_close: Option::Some(security_quote.prev_close),
            volume: Option::Some(security_quote.volume.into()),
            turnover: Option::Some(security_quote.turnover),
            extra: Option::None,
        }
    }

    fn to_quote_basic_info(
        symbol: Symbol,
        security_static_info: SecurityStaticInfo,
    ) -> QuoteBasicInfo {
        QuoteBasicInfo {
            symbol,
            currency: security_static_info.currency.parse().ok(),
            lot_size: security_static_info.lot_size,
            total_shares: security_static_info.total_shares.into(),
            circulating_shares: security_static_info.circulating_shares.into(),
            eps: security_static_info.eps,
            eps_ttm: security_static_info.eps_ttm,
            bps: security_static_info.bps,
            dividend_yield: security_static_info.dividend_yield,
        }
    }

    pub(super) fn to_depth(depth: longbridge::quote::Depth) -> crate::model::trading::quote::Depth {
        crate::model::trading::quote::Depth {
            position: Option::Some(depth.position.into()),
            price: depth.price,
            volume: Option::Some(depth.volume.into()),
            order_count: Option::Some(depth.order_num.into()),
        }
    }

    fn to_quote_depth_info(symbol: Symbol, security_depth: SecurityDepth) -> QuoteDepthInfo {
        let current_timestamp = get_now_unix_timestamp();

        QuoteDepthInfo {
            symbol,
            sequence: current_timestamp,
            timestamp: current_timestamp,
            ask_list: security_depth
                .asks
                .into_iter()
                .map(Self::to_depth)
                .collect(),
            bid_list: security_depth
                .bids
                .into_iter()
                .map(Self::to_depth)
                .collect(),
        }
    }

    fn get_missing_element_error() -> Error {
        anyhow!("longbridge_api_internal_error: Missing elements from the api response.")
    }

    async fn get_longbridge_quote_context(&self) -> longbridge::QuoteContext {
        let (longbridge_quote_context, _) = LongBridgeBroker::create_quote_context().await.unwrap();
        longbridge_quote_context
    }
}

#[async_trait]
impl InfoTrait for LongBridgeInfo {
    fn new(_config_map: ConfigMap) -> Self {
        LongBridgeInfo {}
    }

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        let symbol_identifier = request.symbol.to_string();
        self.get_longbridge_quote_context()
            .await
            .static_info([symbol_identifier])
            .await
            .with_context(|| format!("Error when querying basic info {:?}", request))
            .and_then(|result_vec| match result_vec.into_iter().nth(0) {
                Some(static_info) => {
                    Result::Ok(Self::to_quote_basic_info(request.symbol, static_info))
                }
                None => Result::Err(Self::get_missing_element_error()),
            })
    }

    async fn query_real_time_info(
        &self,
        request: QueryInfoRequest,
    ) -> Result<QuoteRealTimeInfo, Error> {
        // todo: support option
        let symbol_identifier = request.symbol.to_string();
        self.get_longbridge_quote_context()
            .await
            .quote([symbol_identifier])
            .await
            .with_context(|| format!("Error when querying real time info {:?}", request))
            .and_then(|result_vec| match result_vec.into_iter().nth(0) {
                Some(real_time_info) => Result::Ok(Self::to_quote_real_time_info(
                    request.symbol,
                    real_time_info,
                )),
                None => Result::Err(Self::get_missing_element_error()),
            })
    }

    async fn query_depth(&self, request: QueryInfoRequest) -> Result<QuoteDepthInfo, Error> {
        let symbol_identifier = request.symbol.to_string();
        self.get_longbridge_quote_context()
            .await
            .depth(symbol_identifier)
            .await
            .map(|depth_info| Self::to_quote_depth_info(request.symbol.clone(), depth_info))
            .with_context(|| format!("Error when querying depth info {:?}", request))
    }
}
