use anyhow::{anyhow, Context, Error};
use async_trait::async_trait;
use longbridge::trade::{
    AccountBalance, EstimateMaxPurchaseQuantityOptions, EstimateMaxPurchaseQuantityResponse,
    OrderSide, OrderType, OutsideRTH, ReplaceOrderOptions, StockPosition, SubmitOrderOptions,
    TimeInForceType,
};
use rust_decimal::prelude::ToPrimitive;
use time::Date;

use super::broker::LongBridgeBroker;
use crate::{
    broker::common::transaction::TransactionTrait,
    model::{
        common::types::ConfigMap,
        trading::{
            balance::{BalanceDetail, BalanceHashMap},
            position::PositionList,
            transaction::{
                BuyingPower, CancelOrderRequest, CancelOrderResponse, Direction, EditOrderRequest,
                EditOrderResponse, EstimateMaxBuyingPowerRequest, Expire, OrderDetail,
                OrderDetailRequest, Price, RegularTradingTime, SubmitOrderRequest,
                SubmitOrderResponse, TrailingLimitPrice, TrailingMarketPrice,
            },
        },
    },
};

pub struct LongBridgeTransaction {}

impl LongBridgeTransaction {
    fn to_order_side(direction: &Direction) -> OrderSide {
        match direction {
            Direction::Buy => OrderSide::Buy,
            Direction::Sell => OrderSide::Sell,
        }
    }

    fn to_order_type(price: &Price) -> OrderType {
        match price {
            Price::LimitOrder { .. } => OrderType::LO,
            Price::MarketOrder { .. } => OrderType::MO,
            Price::LimitIfTouched { .. } => OrderType::LIT,
            Price::MarketIfTouched { .. } => OrderType::MIT,
            Price::TrailingLimitIfTouched { trailing } => match trailing {
                TrailingLimitPrice::Amount { .. } => OrderType::TSLPAMT,
                TrailingLimitPrice::Percent { .. } => OrderType::TSLPPCT,
            },
            Price::TrailingMarketIfTouched { trailing } => match trailing {
                TrailingMarketPrice::Amount { .. } => OrderType::TSMAMT,
                TrailingMarketPrice::Percent { .. } => OrderType::TSMPCT,
            },
        }
    }

    fn to_time_in_force(expire: &Expire) -> TimeInForceType {
        match expire {
            Expire::Day => TimeInForceType::Day,
            Expire::GoodTillDate { .. } => TimeInForceType::GoodTilDate,
            Expire::GoodTillCancelled => TimeInForceType::GoodTilCanceled,
        }
    }

    fn to_submit_order_options(request: &SubmitOrderRequest) -> SubmitOrderOptions {
        let mut submit_order_options_builder = SubmitOrderOptions::new(
            request.symbol.to_string(),
            Self::to_order_type(&request.price),
            Self::to_order_side(&request.direction),
            request.quantity.to_i64().unwrap(),
            Self::to_time_in_force(&request.expire),
        );

        submit_order_options_builder = match request.expire {
            Expire::GoodTillDate { year, month, day } => submit_order_options_builder.expire_date(
                Date::from_calendar_date(
                    year,
                    time::Month::January.nth_next(month as u8 - 1),
                    day as u8,
                )
                .unwrap(),
            ),
            _ => submit_order_options_builder,
        };

        submit_order_options_builder = match &request.price {
            Price::LimitOrder { price } => {
                submit_order_options_builder.submitted_price(price.clone())
            }

            Price::MarketOrder => submit_order_options_builder,
            Price::LimitIfTouched {
                submit_price,
                trigger_price,
            } => submit_order_options_builder
                .submitted_price(submit_price.clone())
                .trigger_price(trigger_price.clone()),

            Price::MarketIfTouched { trigger_price } => {
                submit_order_options_builder.trigger_price(trigger_price.clone())
            }
            Price::TrailingLimitIfTouched { trailing } => match trailing {
                TrailingLimitPrice::Amount {
                    limit_offset,
                    trailing_amount,
                } => submit_order_options_builder
                    .limit_offset(limit_offset.clone())
                    .trailing_amount(trailing_amount.clone()),
                TrailingLimitPrice::Percent {
                    limit_offset,
                    trailing_percent,
                } => submit_order_options_builder
                    .limit_offset(limit_offset.clone())
                    .trailing_percent(trailing_percent.clone()),
            },
            Price::TrailingMarketIfTouched { trailing } => match trailing {
                TrailingMarketPrice::Amount { trailing_amount } => {
                    submit_order_options_builder.trailing_amount(trailing_amount.clone())
                }
                TrailingMarketPrice::Percent { trailing_percent } => {
                    submit_order_options_builder.trailing_percent(trailing_percent.clone())
                }
            },
        };

        submit_order_options_builder
    }

    fn to_replace_order_options(request: &EditOrderRequest) -> ReplaceOrderOptions {
        let mut replace_order_options_builder =
            ReplaceOrderOptions::new(request.order_id.clone(), request.quantity.to_i64().unwrap());

        replace_order_options_builder = match &request.price {
            Price::LimitOrder { price } => replace_order_options_builder.price(price.clone()),

            Price::MarketOrder => replace_order_options_builder,
            Price::LimitIfTouched {
                submit_price,
                trigger_price,
            } => replace_order_options_builder
                .price(submit_price.clone())
                .trigger_price(trigger_price.clone()),

            Price::MarketIfTouched { trigger_price } => {
                replace_order_options_builder.trigger_price(trigger_price.clone())
            }
            Price::TrailingLimitIfTouched { trailing } => match trailing {
                TrailingLimitPrice::Amount {
                    limit_offset,
                    trailing_amount,
                } => replace_order_options_builder
                    .limit_offset(limit_offset.clone())
                    .trailing_amount(trailing_amount.clone()),
                TrailingLimitPrice::Percent {
                    limit_offset,
                    trailing_percent,
                } => replace_order_options_builder
                    .limit_offset(limit_offset.clone())
                    .trailing_percent(trailing_percent.clone()),
            },
            Price::TrailingMarketIfTouched { trailing } => match trailing {
                TrailingMarketPrice::Amount { trailing_amount } => {
                    replace_order_options_builder.trailing_amount(trailing_amount.clone())
                }
                TrailingMarketPrice::Percent { trailing_percent } => {
                    replace_order_options_builder.trailing_percent(trailing_percent.clone())
                }
            },
        };

        replace_order_options_builder
    }

    fn to_submit_order_response(
        longbridge_response: longbridge::trade::SubmitOrderResponse,
    ) -> SubmitOrderResponse {
        SubmitOrderResponse {
            order_id: longbridge_response.order_id,
        }
    }

    fn to_buying_power(response: EstimateMaxPurchaseQuantityResponse) -> BuyingPower {
        BuyingPower {
            cash_max_quantity: response.cash_max_qty.into(),
            margin_max_quantity: response.margin_max_qty.into(),
        }
    }

    fn to_estimate_max_purchase_quantity_options(
        request: EstimateMaxBuyingPowerRequest,
    ) -> EstimateMaxPurchaseQuantityOptions {
        let mut builder = EstimateMaxPurchaseQuantityOptions::new(
            request.symbol.to_string(),
            Self::to_order_type(&request.price),
            Self::to_order_side(&request.direction),
        );

        builder = match request.price {
            Price::LimitOrder { price } => builder.price(price),
            Price::LimitIfTouched { submit_price, .. } => builder.price(submit_price),
            Price::MarketIfTouched { trigger_price } => builder.price(trigger_price),
            Price::MarketOrder
            | Price::TrailingLimitIfTouched { .. }
            | Price::TrailingMarketIfTouched { .. } => builder,
        };
        builder
    }

    fn to_balance_detail(account_balance: &AccountBalance) -> BalanceDetail {
        BalanceDetail {
            total_cash: account_balance.total_cash,
            net_assets: account_balance.net_assets,
            margin_call: account_balance.margin_call,
            init_margin: account_balance.init_margin,
            maintenance_margin: account_balance.maintenance_margin,
        }
    }

    fn to_stock_position(
        longbridge_position: &StockPosition,
    ) -> Result<crate::model::trading::position::Position, Error> {
        let symbol = LongBridgeBroker::to_symbol(&longbridge_position.symbol)?;
        let currency = LongBridgeBroker::to_currency(&longbridge_position.currency)?;
        Result::Ok(crate::model::trading::position::Position {
            symbol,
            currency,
            cost_price: longbridge_position.cost_price,
            total_quantity: longbridge_position.quantity.into(),
            available_quantity: longbridge_position.available_quantity.into(),
        })
    }

    fn to_order_direction(order_side: OrderSide) -> Result<Direction, Error> {
        match order_side {
            OrderSide::Buy => Result::Ok(Direction::Buy),
            OrderSide::Sell => Result::Ok(Direction::Sell),
            OrderSide::Unknown => Result::Err(anyhow!("PARSING_ERROR UNKNOWN_ORDER_SIDE")),
        }
    }

    fn to_order_detail_response(
        longbridge_order_detail: longbridge::trade::OrderDetail,
    ) -> Result<OrderDetail, Error> {
        let symbol = LongBridgeBroker::to_symbol(&longbridge_order_detail.symbol)?;
        let currency = LongBridgeBroker::to_currency(&longbridge_order_detail.currency)?;
        let direction = Self::to_order_direction(longbridge_order_detail.side)?;
        let regular_trading_time = match longbridge_order_detail
            .outside_rth
            .unwrap_or(OutsideRTH::AnyTime)
        {
            OutsideRTH::Unknown => Result::Err(anyhow!("PARSING_ERROR UNKNOWN_OUTSIDE_RTH"))?,
            OutsideRTH::RTHOnly => RegularTradingTime::OnlyRegularTradingTime,
            OutsideRTH::AnyTime => RegularTradingTime::AllTime,
        };
        let expire = match longbridge_order_detail.time_in_force {
            TimeInForceType::Unknown => {
                Result::Err(anyhow!("PARSING_ERROR UNKNOWN_TIME_IN_FORCE"))?
            }
            TimeInForceType::Day => Expire::Day,
            TimeInForceType::GoodTilCanceled => Expire::GoodTillCancelled,
            TimeInForceType::GoodTilDate => {
                let order_expire_date = longbridge_order_detail.expire_date.unwrap();
                Expire::GoodTillDate {
                    year: order_expire_date.clone().year(),
                    month: order_expire_date.clone().month() as i32,
                    day: order_expire_date.clone().day() as i32,
                }
            }
        };
        let price: Price = match longbridge_order_detail.order_type {
            OrderType::LO => Price::LimitOrder {
                price: longbridge_order_detail.price.unwrap(),
            },
            OrderType::MO => Price::MarketOrder {},
            OrderType::LIT => Price::LimitIfTouched {
                submit_price: longbridge_order_detail.price.unwrap(),
                trigger_price: longbridge_order_detail.trigger_price.unwrap(),
            },
            OrderType::MIT => Price::MarketIfTouched {
                trigger_price: longbridge_order_detail.trigger_price.unwrap(),
            },
            OrderType::TSLPAMT => Price::TrailingLimitIfTouched {
                trailing: TrailingLimitPrice::Amount {
                    limit_offset: longbridge_order_detail.limit_offset.unwrap(),
                    trailing_amount: longbridge_order_detail.trailing_amount.unwrap(),
                },
            },
            OrderType::TSLPPCT => Price::TrailingLimitIfTouched {
                trailing: TrailingLimitPrice::Percent {
                    limit_offset: longbridge_order_detail.limit_offset.unwrap(),
                    trailing_percent: longbridge_order_detail.trailing_percent.unwrap(),
                },
            },
            OrderType::TSMAMT => Price::TrailingMarketIfTouched {
                trailing: TrailingMarketPrice::Amount {
                    trailing_amount: longbridge_order_detail.trailing_amount.unwrap(),
                },
            },
            OrderType::TSMPCT => Price::TrailingMarketIfTouched {
                trailing: TrailingMarketPrice::Percent {
                    trailing_percent: longbridge_order_detail.trailing_percent.unwrap(),
                },
            },
            OrderType::Unknown
            | OrderType::ELO
            | OrderType::AO
            | OrderType::ALO
            | OrderType::ODD
            | OrderType::SLO => Result::Err(anyhow!("PARSING_ERROR UNKNOWN_ORDER_TYPE_MESSAGE"))?,
        };

        let order_detail = OrderDetail {
            order_id: longbridge_order_detail.order_id,
            symbol,
            currency,
            quantity: longbridge_order_detail.quantity.into(),
            executed_quantity: longbridge_order_detail.executed_quantity.into(),
            price,
            executed_price: longbridge_order_detail.executed_price,
            direction,
            regular_trading_time,
            expire,
            created_timestamp: longbridge_order_detail.submitted_at.unix_timestamp() as u64,
            updated_timestamp: longbridge_order_detail
                .updated_at
                .map(|timestamp| timestamp.unix_timestamp() as u64),
            triggered_timestamp: longbridge_order_detail
                .trigger_at
                .map(|timestamp| timestamp.unix_timestamp() as u64),
        };
        Result::Ok(order_detail)
    }

    async fn get_longbridge_trade_context(&self) -> longbridge::TradeContext {
        let (longbridge_quote_context, _) = LongBridgeBroker::create_trade_context().await.unwrap();
        longbridge_quote_context
    }
}

#[async_trait]
impl TransactionTrait for LongBridgeTransaction {
    fn new(_config_map: ConfigMap) -> Self {
        LongBridgeTransaction {}
    }

    async fn submit_order(
        &mut self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error> {
        self.get_longbridge_trade_context()
            .await
            .submit_order(Self::to_submit_order_options(&request))
            .await
            .map(Self::to_submit_order_response)
            .with_context(|| format!("Error when calling submit_order, request: {:?}", request))
    }

    async fn edit_order(&mut self, request: EditOrderRequest) -> Result<EditOrderResponse, Error> {
        self.get_longbridge_trade_context()
            .await
            .replace_order(Self::to_replace_order_options(&request))
            .await
            .map(|_| EditOrderResponse {})
            .with_context(|| format!("Error when calling edit_order, request: {:?}", request))
    }

    async fn cancel_order(
        &mut self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error> {
        self.get_longbridge_trade_context()
            .await
            .cancel_order(request.order_id.clone())
            .await
            .map(|_| CancelOrderResponse {})
            .with_context(|| format!("Error when calling cancel_order, request: {:?}", request))
    }

    async fn estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error> {
        self.get_longbridge_trade_context()
            .await
            .estimate_max_purchase_quantity(Self::to_estimate_max_purchase_quantity_options(
                request.clone(),
            ))
            .await
            .map(Self::to_buying_power)
            .with_context(|| {
                format!(
                    "Error when calling estimate_max_buying_power, request: {:?}",
                    request
                )
            })
    }

    async fn order_detail(&self, request: OrderDetailRequest) -> Result<OrderDetail, Error> {
        self.get_longbridge_trade_context()
            .await
            .order_detail(request.order_id.clone())
            .await
            .with_context(|| format!("Error when calling order_detail, request: {:?}", request))
            .and_then(Self::to_order_detail_response)
    }

    async fn account_balance(&self) -> Result<BalanceHashMap, Error> {
        self.get_longbridge_trade_context()
            .await
            .account_balance(Option::None)
            .await
            .map(|currencies_balance| {
                currencies_balance
                    .into_iter()
                    .map(|account_balance| {
                        (
                            LongBridgeBroker::to_currency(account_balance.currency.as_str()),
                            Self::to_balance_detail(&account_balance),
                        )
                    })
                    .filter_map(|entry| {
                        let (key_result, value) = entry;
                        let key = key_result.ok()?;
                        Option::Some((key, value))
                    })
                    .collect()
            })
            .with_context(|| format!("Error when calling account_balance"))
    }

    async fn positions(&self) -> Result<PositionList, Error> {
        self.get_longbridge_trade_context()
            .await
            .stock_positions(Option::None)
            .await
            .map(|response| {
                response
                    .channels
                    .iter()
                    .flat_map(|stock_position_channel| &stock_position_channel.positions)
                    .filter_map(|position| Self::to_stock_position(position).ok())
                    .collect()
            })
            .with_context(|| format!("Error when calling positions"))
    }
}
