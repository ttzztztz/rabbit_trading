use async_trait::async_trait;
use longbridge::{
    trade::{
        AccountBalance, EstimateMaxPurchaseQuantityOptions, EstimateMaxPurchaseQuantityResponse,
        OrderSide, OrderType, StockPosition, SubmitOrderOptions, TimeInForceType,
    },
    TradeContext,
};
use time::Date;

use super::broker::LongBridgeBroker;
use crate::{
    broker::common::transaction::TransactionTrait,
    model::{
        balance::{BalanceDetail, BalanceHashMap},
        error::Error,
        position::PositionList,
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
        long_bridge_position: &StockPosition,
    ) -> Option<crate::model::position::Position> {
        let symbol = LongBridgeBroker::to_symbol(&long_bridge_position.symbol)?;
        let currency = LongBridgeBroker::to_currency(&long_bridge_position.currency)?;
        Option::Some(crate::model::position::Position {
            symbol,
            currency,
            cost_price: long_bridge_position.cost_price,
            total_quantity: long_bridge_position.quantity,
            available_quantity: long_bridge_position.available_quantity,
        })
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

    async fn account_balance(&self) -> Result<BalanceHashMap, Error> {
        self.longbridge_context
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
                        let (key, value) = entry;
                        key.as_ref()?;
                        Option::Some((key.unwrap(), value))
                    })
                    .collect()
            })
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
    }

    async fn positions(&self) -> Result<PositionList, Error> {
        self.longbridge_context
            .stock_positions(Option::None)
            .await
            .map(|response| {
                response
                    .channels
                    .iter()
                    .flat_map(|stock_position_channel| &stock_position_channel.positions)
                    .filter_map(Self::to_stock_position)
                    .collect()
            })
            .map_err(LongBridgeBroker::to_rabbit_trading_err)
    }
}

#[cfg(test)]
mod test_long_bridge_transaction {
    use rust_decimal_macros::dec;

    use super::LongBridgeTransaction;
    use crate::{broker::common::transaction::TransactionTrait, model::currency::Currency};

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_account_balance() {
        let long_bridge_transaction = LongBridgeTransaction::new().await;

        let account_balance_result = long_bridge_transaction.account_balance().await;
        assert!(account_balance_result.is_ok());
        let account_balance_map = account_balance_result.unwrap();
        let account_balance_hkd_option = account_balance_map.get(&Currency::HKD);
        assert!(account_balance_hkd_option.is_some());
        let account_balance_hkd = account_balance_hkd_option.unwrap();
        assert!(account_balance_hkd.total_cash >= dec!(0.0));
        assert!(account_balance_hkd.init_margin >= dec!(0.0));
        assert!(account_balance_hkd.maintenance_margin >= dec!(0.0));
        assert!(account_balance_hkd.margin_call >= dec!(0.0));
        assert!(account_balance_hkd.net_assets >= dec!(0.0));
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_positions() {
        let long_bridge_transaction = LongBridgeTransaction::new().await;

        let positions_result = long_bridge_transaction.positions().await;
        assert!(positions_result.is_ok());
        positions_result.unwrap().iter().for_each(|position| {
            assert!(position.cost_price >= dec!(0));
            assert!(position.available_quantity >= 0);
            assert!(position.total_quantity >= 0);
        });
    }
}
