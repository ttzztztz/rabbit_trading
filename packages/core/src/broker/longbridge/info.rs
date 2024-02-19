use async_trait::async_trait;
use longbridge::quote::{SecurityDepth, SecurityQuote, SecurityStaticInfo};
use longbridge::QuoteContext;
use std::result::Result;

use super::broker::LongBridgeBroker;
use crate::broker::common::info::InfoTrait;
use crate::model::{
    common::{error::Error, types::ConfigMap},
    trading::{
        quote::{QueryInfoRequest, QuoteBasicInfo, QuoteDepthInfo, QuoteRealTimeInfo},
        symbol::Symbol,
    },
};
use crate::utils::time::get_now_unix_timestamp;

pub struct LongBridgeInfo {
    longbridge_context: QuoteContext,
}

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
            volume: security_quote.volume as u64,
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
            currency: LongBridgeBroker::to_currency(&security_static_info.currency).ok(),
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
            position: depth.position.into(),
            price: depth.price,
            volume: depth.volume.into(),
            order_count: depth.order_num.into(),
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
        const ERROR_CODE: &'static str = "longbridge_api_internal_error";
        const MESSAGE: &'static str = "Missing elements from the api response.";

        Error {
            code: ERROR_CODE.to_owned(),
            message: MESSAGE.to_owned(),
        }
    }
}

#[async_trait]
impl InfoTrait for LongBridgeInfo {
    async fn new(_config_map: ConfigMap) -> Self {
        let (longbridge_context, _) = LongBridgeBroker::create_quote_context().await.unwrap();
        LongBridgeInfo { longbridge_context }
    }

    async fn query_basic_info(&self, request: QueryInfoRequest) -> Result<QuoteBasicInfo, Error> {
        let symbol_identifier = request.symbol.to_string();
        self.longbridge_context
            .static_info([symbol_identifier])
            .await
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
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
        self.longbridge_context
            .quote([symbol_identifier])
            .await
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
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
        self.longbridge_context
            .depth(symbol_identifier)
            .await
            .map(|depth_info| Self::to_quote_depth_info(request.symbol, depth_info))
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
    }
}
