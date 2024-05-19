use anyhow::{anyhow, Context, Error};
use async_trait::async_trait;
use ibkr_client_portal::{
    client::IBClientPortal,
    model::{
        account::{GetAccountSummaryRequest, GetAccountSummaryResponse},
        order::{
            CancelOrderRequest as IBCancelOrderRequest, GetOrderStatusRequest, ModifyOrderRequest,
            OrderRequest, OrderStatus, PlaceOrdersRequest,
        },
        portfolio::GetPortfolioPositionsRequest,
    },
    utils::reply::handle_reply_order_requests,
};
use rust_decimal_macros::dec;
use std::time::SystemTime;

use super::{broker::InteractiveBrokersBroker, config::IBConfig, symbol::IBSymbolHelper};
use crate::{
    broker::common::transaction::TransactionTrait,
    model::{
        common::types::ConfigMap,
        trading::{
            balance::{BalanceDetail, BalanceHashMap},
            currency::Currency,
            position::PositionList,
            transaction::{
                BuyingPower, CancelOrderRequest, CancelOrderResponse, Direction, EditOrderRequest,
                EditOrderResponse, EstimateMaxBuyingPowerRequest, Expire, OrderDetail,
                OrderDetailRequest, Price, SubmitOrderRequest, SubmitOrderResponse,
                TrailingLimitPrice, TrailingMarketPrice,
            },
        },
    },
};

pub struct InteractiveBrokersTransaction {
    config_map: ConfigMap,
    client_portal: IBClientPortal,
    ib_symbol_helper: IBSymbolHelper,
}

impl InteractiveBrokersTransaction {
    fn get_account_summary_response_to_balance_hashmap(
        account_summary: GetAccountSummaryResponse,
    ) -> BalanceHashMap {
        let total_cash = account_summary
            .full_available_funds
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));
        let init_margin = account_summary
            .init_margin_req
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));
        let maintenance_margin = account_summary
            .maintenance_marginreq
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));
        let net_assets = account_summary
            .net_liquidation
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));

        let balance_detail = BalanceDetail {
            total_cash,
            net_assets,
            margin_call: dec!(0.0), // TODO
            init_margin,
            maintenance_margin,
        };
        BalanceHashMap::from([(Currency::USD, balance_detail)]) // TODO: support other currencies
    }

    fn get_account_summary_response_to_buying_power(
        account_summary: GetAccountSummaryResponse,
    ) -> BuyingPower {
        let cash_max_quantity = account_summary
            .full_available_funds
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));
        let margin_max_quantity = account_summary
            .buying_power
            .and_then(|funds| funds.amount)
            .unwrap_or(dec!(0.0));

        BuyingPower {
            cash_max_quantity,
            margin_max_quantity,
        }
    }

    async fn ib_position_to_core_position(
        &self,
        position: &ibkr_client_portal::model::portfolio::Position,
    ) -> Result<crate::model::trading::position::Position, Error> {
        let symbol = self
            .ib_symbol_helper
            .get_symbol(
                position.conid.clone().unwrap().parse().unwrap(), // TODO: eliminate the unwrap() call here
            )
            .unwrap();
        let currency =
            InteractiveBrokersBroker::to_currency(position.currency.clone().unwrap().as_str())?;

        Result::Ok(crate::model::trading::position::Position {
            symbol,
            currency,
            cost_price: position.avg_cost,
            quantity: position.position,
        })
    }

    async fn ib_order_status_to_core_order_detail(
        &self,
        order_status: OrderStatus,
    ) -> Result<OrderDetail, Error> {
        let order_id = order_status.order_id.unwrap().to_string();
        let symbol = self
            .ib_symbol_helper
            .get_symbol(
                order_status.conid.clone().unwrap(), // TODO: eliminate the unwrap() call here
            )
            .unwrap();
        let currency =
            InteractiveBrokersBroker::to_currency(order_status.currency.clone().unwrap().as_str())?;
        let regular_trading_time = match order_status.outside_regular_trading_hours.unwrap() {
            true => crate::model::trading::transaction::RegularTradingTime::AllTime,
            false => crate::model::trading::transaction::RegularTradingTime::OnlyRegularTradingTime,
        };
        let direction = Self::side_to_direction(order_status.side.clone().unwrap().as_str())?;
        let expire =
            Self::time_in_force_to_expire(order_status.time_in_force.clone().unwrap().as_str())?;
        let price: Price = match order_status.order_type.clone().unwrap().as_str() {
            "LMT" | "LIMIT" => Price::LimitOrder {
                price: order_status.limit_price.unwrap().clone(),
            },
            "MKT" | "MARKET" => Price::MarketOrder,
            "STP" => Price::MarketIfTouched {
                trigger_price: order_status.stop_price.unwrap().clone(),
            },
            "STOP_LIMIT" => Price::LimitIfTouched {
                submit_price: order_status.limit_price.unwrap().clone(),
                trigger_price: order_status.stop_price.unwrap().clone(),
            },
            "TRAIL" | "TRAILING_STOP" => Price::TrailingMarketIfTouched {
                trailing: match order_status.trailing_amount_unit.clone().unwrap().as_str() {
                    "amt" => TrailingMarketPrice::Amount {
                        trailing_amount: order_status.trailing_amount.unwrap(),
                    },
                    "%" => TrailingMarketPrice::Percent {
                        trailing_percent: order_status.trailing_amount.unwrap(),
                    },
                    _ => Result::Err(anyhow!(
                        "Error, unsupported trailing_amount_unit {:?}",
                        order_status
                    ))?,
                },
            },
            "TRAILLMT" | "TRAILING_STOP_LIMIT" => Price::TrailingLimitIfTouched {
                trailing: match order_status.trailing_amount_unit.clone().unwrap().as_str() {
                    "amt" => TrailingLimitPrice::Amount {
                        limit_offset: order_status.limit_price_offset.unwrap(),
                        trailing_amount: order_status.trailing_amount.unwrap(),
                    },
                    "%" => TrailingLimitPrice::Percent {
                        limit_offset: order_status.limit_price_offset.unwrap(),
                        trailing_percent: order_status.trailing_amount.unwrap(),
                    },
                    _ => Result::Err(anyhow!(
                        "Error, unsupported trailing_amount_unit {:?}",
                        order_status
                    ))?,
                },
            },
            _ => Result::Err(anyhow!("Error, unsupported order_type {:?}", order_status))?,
        };

        Result::Ok(OrderDetail {
            order_id,
            symbol,
            currency,
            quantity: order_status.total_size.unwrap(),
            executed_quantity: order_status.cum_fill.unwrap(),
            price,
            executed_price: Option::None, // TODO
            direction,
            regular_trading_time,
            expire,
            created_timestamp: order_status.order_time.unwrap().parse().unwrap(),
            updated_timestamp: Option::None,
            triggered_timestamp: Option::None,
        })
    }

    fn core_edit_order_request_to_ib_modify_order_request(
        account_id: String,
        request: EditOrderRequest,
    ) -> ModifyOrderRequest {
        let mut modify_order_request = ModifyOrderRequest {
            account_id_or_financial_advisors_group: account_id.clone(),
            order_id: request.order_id.clone(),
            account_id: Option::Some(account_id),
            conid: Option::None,
            conidex: Option::None, // TODO: validate
            order_type: Option::Some("LMT".to_owned()),
            outside_regular_trading_hours: Option::None,
            price: Option::None,
            aux_price: Option::None,
            side: Option::None, // TODO: validate
            listing_exchange: Option::None,
            ticker: Option::None,
            time_in_force: Option::None, // TODO: validate
            quantity: Option::Some(request.quantity),
            deactivated: Option::None,
            use_adaptive: Option::Some(false),
            limit_offset: Option::None,
            trailing_amount: Option::None,
            trailing_type: Option::None,
        };

        match &request.price {
            Price::LimitOrder { price } => {
                modify_order_request.order_type = Option::Some("LIMIT".to_owned());
                modify_order_request.price = Option::Some(price.clone());
            }

            Price::MarketOrder => {
                modify_order_request.order_type = Option::Some("MARKET".to_owned());
            }
            Price::LimitIfTouched {
                submit_price,
                trigger_price,
            } => {
                modify_order_request.order_type = Option::Some("STOP_LIMIT".to_owned());
                modify_order_request.price = Option::Some(submit_price.clone());
                modify_order_request.aux_price = Option::Some(trigger_price.clone());
            }

            Price::MarketIfTouched { trigger_price } => {
                modify_order_request.order_type = Option::Some("STP".to_owned());
                modify_order_request.price = Option::Some(trigger_price.clone());
            }
            Price::TrailingLimitIfTouched { trailing } => match trailing {
                TrailingLimitPrice::Amount {
                    limit_offset,
                    trailing_amount,
                } => {
                    modify_order_request.order_type = Option::Some("TRAILING_STOP".to_owned());
                    modify_order_request.trailing_type = Option::Some("amt".to_owned());
                    modify_order_request.trailing_amount = Option::Some(trailing_amount.clone());
                    modify_order_request.limit_offset = Option::Some(limit_offset.clone());
                }
                TrailingLimitPrice::Percent {
                    limit_offset,
                    trailing_percent,
                } => {
                    modify_order_request.order_type = Option::Some("TRAILING_STOP".to_owned());
                    modify_order_request.trailing_type = Option::Some("%".to_owned());
                    modify_order_request.trailing_amount = Option::Some(trailing_percent.clone());
                    modify_order_request.limit_offset = Option::Some(limit_offset.clone());
                }
            },
            Price::TrailingMarketIfTouched { trailing } => match trailing {
                TrailingMarketPrice::Amount { trailing_amount } => {
                    modify_order_request.order_type =
                        Option::Some("TRAILING_STOP_LIMIT".to_owned());
                    modify_order_request.trailing_type = Option::Some("amt".to_owned());
                    modify_order_request.trailing_amount = Option::Some(trailing_amount.clone());
                }
                TrailingMarketPrice::Percent { trailing_percent } => {
                    modify_order_request.order_type =
                        Option::Some("TRAILING_STOP_LIMIT".to_owned());
                    modify_order_request.trailing_type = Option::Some("%".to_owned());
                    modify_order_request.trailing_amount = Option::Some(trailing_percent.clone());
                }
            },
        };
        modify_order_request
    }

    async fn core_submit_order_request_to_ib_order(
        &self,
        account_id: String,
        request: SubmitOrderRequest,
    ) -> Result<OrderRequest, Error> {
        let conid = self.ib_symbol_helper.get_conid(&request.symbol).unwrap();
        let outside_regular_trading_hours = match request.regular_trading_time {
            crate::model::trading::transaction::RegularTradingTime::AllTime => true,
            crate::model::trading::transaction::RegularTradingTime::OnlyRegularTradingTime => false,
        };
        let mut order_request = OrderRequest {
            account_id: Option::Some(account_id),
            conid: Option::None,
            conidex: Option::Some(conid.to_string()),
            sec_type: Option::None,
            c_oid: Option::Some(Self::generate_customer_order_id()),
            parent_id: Option::None,
            order_type: "LMT".to_owned(),
            limit_offset: Option::None,
            listing_exchange: Option::None,
            is_single_group: Option::None,
            outside_regular_trading_hours,
            price: Option::None,
            aux_price: Option::None,
            side: Self::direction_to_side(request.direction),
            ticker: Option::None,
            time_in_force: Self::expire_to_time_in_force(request.expire)?,
            trailing_amount: Option::None,
            trailing_type: Option::None,
            referrer: Option::None,
            quantity: Option::Some(request.quantity),
            cash_quantity: Option::None,
            fx_quantity: Option::None,
            use_adaptive: Option::Some(false),
            is_currency_conv: Option::None,
            allocation_method: Option::None,
            strategy: Option::None,
            strategy_parameters: Option::None,
            originator: Option::Some("Side-Order".to_owned()),
        };

        // TODO: merge into one macro
        match &request.price {
            Price::LimitOrder { price } => {
                order_request.order_type = "LMT".to_owned();
                order_request.price = Option::Some(price.clone());
            }

            Price::MarketOrder => {
                order_request.order_type = "MKT".to_owned();
            }
            Price::LimitIfTouched {
                submit_price,
                trigger_price,
            } => {
                order_request.order_type = "STOP_LIMIT".to_owned();
                order_request.price = Option::Some(submit_price.clone());
                order_request.aux_price = Option::Some(trigger_price.clone());
            }

            Price::MarketIfTouched { trigger_price } => {
                order_request.order_type = "STP".to_owned();
                order_request.price = Option::Some(trigger_price.clone());
            }
            Price::TrailingLimitIfTouched { trailing } => match trailing {
                TrailingLimitPrice::Amount {
                    limit_offset,
                    trailing_amount,
                } => {
                    order_request.order_type = "TRAILLMT".to_owned();
                    order_request.trailing_type = Option::Some("amt".to_owned());
                    order_request.trailing_amount = Option::Some(trailing_amount.clone());
                    order_request.limit_offset = Option::Some(limit_offset.clone());
                }
                TrailingLimitPrice::Percent {
                    limit_offset,
                    trailing_percent,
                } => {
                    order_request.order_type = "TRAILLMT".to_owned();
                    order_request.trailing_type = Option::Some("%".to_owned());
                    order_request.trailing_amount = Option::Some(trailing_percent.clone());
                    order_request.limit_offset = Option::Some(limit_offset.clone());
                }
            },
            Price::TrailingMarketIfTouched { trailing } => match trailing {
                TrailingMarketPrice::Amount { trailing_amount } => {
                    order_request.order_type = "TRAILING_STOP".to_owned();
                    order_request.trailing_type = Option::Some("amt".to_owned());
                    order_request.trailing_amount = Option::Some(trailing_amount.clone());
                }
                TrailingMarketPrice::Percent { trailing_percent } => {
                    order_request.order_type = "TRAILING_STOP".to_owned();
                    order_request.trailing_type = Option::Some("%".to_owned());
                    order_request.trailing_amount = Option::Some(trailing_percent.clone());
                }
            },
        };
        Result::Ok(order_request)
    }

    fn side_to_direction(side: &str) -> Result<Direction, Error> {
        match side {
            "B" => Result::Ok(Direction::Buy),
            "S" => Result::Ok(Direction::Sell),
            _ => Result::Err(anyhow!(
                "Error determining the order direction for {}",
                side
            )),
        }
    }

    fn direction_to_side(direction: Direction) -> String {
        match direction {
            Direction::Buy => "BUY".to_owned(),
            Direction::Sell => "SELL".to_owned(),
        }
    }

    fn time_in_force_to_expire(time_in_force: &str) -> Result<Expire, Error> {
        match time_in_force {
            "GTC" => Result::Ok(Expire::GoodTillCancelled),
            "OPG" => Result::Ok(Expire::OpenPriceGuarantee),
            "DAY" => Result::Ok(Expire::Day),
            "IOC" => Result::Ok(Expire::ImmediateOrCancel),
            _ => Result::Err(anyhow!(
                "Error determining the order expire for {}",
                time_in_force
            )),
        }
    }

    fn expire_to_time_in_force(expire: Expire) -> Result<String, Error> {
        match expire {
            Expire::GoodTillCancelled => Result::Ok("GTC".to_owned()),
            Expire::OpenPriceGuarantee => Result::Ok("OPG".to_owned()),
            Expire::Day => Result::Ok("DAY".to_owned()),
            Expire::ImmediateOrCancel => Result::Ok("IOC".to_owned()),
            _ => Result::Err(anyhow!(
                "Error determining the order time_in_force for {:?} (unsupported)",
                expire
            )),
        }
    }

    fn generate_customer_order_id() -> String {
        // TODO: modularize this function
        format!(
            "rt_{}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64
        )
    }
}

#[async_trait]
impl TransactionTrait for InteractiveBrokersTransaction {
    fn new(config_map: ConfigMap) -> Self {
        let client_portal = InteractiveBrokersBroker::create_ib_client_portal(config_map.clone());
        let ib_config = IBConfig::new(&config_map).unwrap();
        let ib_symbol_helper = IBSymbolHelper::new(ib_config);

        InteractiveBrokersTransaction {
            config_map,
            client_portal,
            ib_symbol_helper,
        }
    }

    async fn account_balance(&self) -> Result<BalanceHashMap, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let account_summary = self
            .client_portal
            .get_account_summary(GetAccountSummaryRequest {
                account_id: account_id.clone(),
            })
            .await
            .with_context(|| format!("Error when account_balance, account_id={}", account_id))?;

        Result::Ok(Self::get_account_summary_response_to_balance_hashmap(
            account_summary,
        ))
    }

    async fn positions(&self) -> Result<PositionList, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let positions = self
            .client_portal
            .get_portfolio_positions(GetPortfolioPositionsRequest { page: 1 }) // TODO: support pagination
            .await
            .with_context(|| format!("Error when retrieve position data {}", account_id))?;

        let mut position_list = vec![];
        for position in positions.iter() {
            position_list.push(self.ib_position_to_core_position(position).await?);
        }
        Result::Ok(position_list)
    }

    async fn estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let account_summary = self
            .client_portal
            .get_account_summary(GetAccountSummaryRequest { account_id })
            .await
            .with_context(|| format!("Error when estimate_max_buying_power {:?}", request))?;

        Result::Ok(Self::get_account_summary_response_to_buying_power(
            account_summary,
        ))
    }

    async fn order_detail(&self, request: OrderDetailRequest) -> Result<OrderDetail, Error> {
        let order_detail = self
            .client_portal
            .get_order_status(GetOrderStatusRequest {
                order_id: request.order_id.clone(),
            })
            .await
            .with_context(|| format!("Error when order_detail {:?}", request))?;

        Result::Ok(
            self.ib_order_status_to_core_order_detail(order_detail)
                .await?,
        )
    }

    async fn submit_order(
        &mut self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let max_retry_count =
            InteractiveBrokersBroker::get_place_order_max_reply_count(&self.config_map);
        let place_order_response = self
            .client_portal
            .place_orders(PlaceOrdersRequest {
                account_id: account_id.clone(),
                orders: vec![
                    self.core_submit_order_request_to_ib_order(account_id, request)
                        .await?,
                ],
            })
            .await?;
        let order_id = handle_reply_order_requests(
            self.client_portal.clone(),
            place_order_response,
            max_retry_count,
        )
        .await?;
        Result::Ok(SubmitOrderResponse { order_id })
    }

    async fn edit_order(&mut self, request: EditOrderRequest) -> Result<EditOrderResponse, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let max_retry_count =
            InteractiveBrokersBroker::get_place_order_max_reply_count(&self.config_map);

        let place_order_response = self
            .client_portal
            .modify_order(Self::core_edit_order_request_to_ib_modify_order_request(
                account_id, request,
            ))
            .await?;
        handle_reply_order_requests(
            self.client_portal.clone(),
            place_order_response,
            max_retry_count,
        )
        .await
        .map(|_| EditOrderResponse {})
    }

    async fn cancel_order(
        &mut self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error> {
        let account_id = InteractiveBrokersBroker::get_account_id(&self.config_map);
        let order_id = request.order_id.clone();
        self.client_portal
            .cancel_order(IBCancelOrderRequest {
                account_id,
                order_id,
            })
            .await
            .map(|_| CancelOrderResponse {})
            .with_context(|| format!("Error when cancelling the order {:?}", request))
    }
}
