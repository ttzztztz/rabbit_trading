use async_trait::async_trait;
use longbridge::{
    trade::{
        EstimateMaxPurchaseQuantityOptions, EstimateMaxPurchaseQuantityResponse, OrderSide,
        OrderType, SubmitOrderOptions, TimeInForceType,
    },
    TradeContext,
};
use time::Date;

use super::broker::LongBridgeBroker;
use crate::{
    broker::common::transaction::TransactionTrait,
    model::{
        error::Error,
        transaction::{
            BuyingPower, Direction, EstimateMaxBuyingPowerRequest, Expire, Price,
            SubmitOrderRequest, SubmitOrderResponse, TrailingLimitPrice, TrailingMarketPrice,
        },
    },
};

pub struct LongBridgeTransaction {
    longbridge_context: TradeContext,
}

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

    fn to_submit_order_options(request: SubmitOrderRequest) -> SubmitOrderOptions {
        let mut submit_order_options_builder = SubmitOrderOptions::new(
            request.symbol.to_string(),
            Self::to_order_type(&request.price),
            Self::to_order_side(&request.direction),
            request.quantity,
            match request.expire {
                Expire::Day => TimeInForceType::Day,
                Expire::GoodTillDate => TimeInForceType::GoodTilDate,
                Expire::GoodTillCancelled { .. } => TimeInForceType::GoodTilCanceled,
            },
        );

        submit_order_options_builder = match request.expire {
            Expire::GoodTillCancelled { year, month, day } => submit_order_options_builder
                .expire_date(
                    Date::from_calendar_date(
                        year,
                        time::Month::January.nth_next(month as u8 - 1),
                        day as u8,
                    )
                    .unwrap(),
                ),
            _ => submit_order_options_builder,
        };

        submit_order_options_builder = match request.price {
            Price::LimitOrder { price } => submit_order_options_builder.submitted_price(price),

            Price::MarketOrder => submit_order_options_builder,
            Price::LimitIfTouched {
                submit_price,
                trigger_price,
            } => submit_order_options_builder
                .submitted_price(submit_price)
                .trigger_price(trigger_price),

            Price::MarketIfTouched { trigger_price } => {
                submit_order_options_builder.trigger_price(trigger_price)
            }
            Price::TrailingLimitIfTouched { trailing } => match trailing {
                TrailingLimitPrice::Amount {
                    limit_offset,
                    trailing_amount,
                } => submit_order_options_builder
                    .limit_offset(limit_offset)
                    .trailing_amount(trailing_amount),
                TrailingLimitPrice::Percent {
                    limit_offset,
                    trailing_percent,
                } => submit_order_options_builder
                    .limit_offset(limit_offset)
                    .trailing_percent(trailing_percent),
            },
            Price::TrailingMarketIfTouched { trailing } => match trailing {
                TrailingMarketPrice::Amount { trailing_amount } => {
                    submit_order_options_builder.trailing_amount(trailing_amount)
                }
                TrailingMarketPrice::Percent { trailing_percent } => {
                    submit_order_options_builder.trailing_percent(trailing_percent)
                }
            },
        };

        submit_order_options_builder
    }

    fn to_submit_order_response(
        long_bridge_response: longbridge::trade::SubmitOrderResponse,
    ) -> SubmitOrderResponse {
        SubmitOrderResponse {
            order_id: long_bridge_response.order_id,
        }
    }

    fn to_buying_power(response: EstimateMaxPurchaseQuantityResponse) -> BuyingPower {
        BuyingPower {
            cash_max_quantity: response.cash_max_qty,
            margin_max_quantity: response.margin_max_qty,
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
}

#[async_trait]
impl TransactionTrait for LongBridgeTransaction {
    async fn new() -> Self {
        let (ctx, _) = LongBridgeBroker::create_trade_context().await.unwrap();

        LongBridgeTransaction {
            longbridge_context: ctx,
        }
    }

    async fn submit_order(
        &self,
        request: SubmitOrderRequest,
    ) -> Result<SubmitOrderResponse, Error> {
        self.longbridge_context
            .submit_order(Self::to_submit_order_options(request))
            .await
            .map(Self::to_submit_order_response)
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
    }

    async fn estimate_max_buying_power(
        &self,
        request: EstimateMaxBuyingPowerRequest,
    ) -> Result<BuyingPower, Error> {
        self.longbridge_context
            .estimate_max_purchase_quantity(Self::to_estimate_max_purchase_quantity_options(
                request,
            ))
            .await
            .map(Self::to_buying_power)
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
    }
}
