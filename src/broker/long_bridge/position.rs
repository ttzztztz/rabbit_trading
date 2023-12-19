use async_trait::async_trait;
use longbridge::{
    trade::{AccountBalance, StockPosition},
    TradeContext,
};

use super::broker::LongBridgeBroker;
use crate::{
    broker::common::position_trait::Position,
    model::{
        balance::{BalanceDetail, BalanceHashMap},
        error::Error,
        position::PositionList,
    },
};

pub struct LongBridgePosition {
    longbridge_context: TradeContext,
}

impl LongBridgePosition {
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
impl Position for LongBridgePosition {
    async fn new() -> Self {
        let (ctx, _) = LongBridgeBroker::create_trade_context().await.unwrap();

        LongBridgePosition {
            longbridge_context: ctx,
        }
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
                            LongBridgeBroker::to_currency(&account_balance.currency.as_str()),
                            Self::to_balance_detail(&account_balance),
                        )
                    })
                    .filter_map(|entry| {
                        let (key, value) = entry;
                        if key.is_none() {
                            return Option::None;
                        }
                        Option::Some((key.unwrap(), value))
                    })
                    .collect()
            })
            .map_err(|long_bridge_err| LongBridgeBroker::to_rabbit_trading_err(long_bridge_err))
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
            .map_err(|long_bridge_err| LongBridgeBroker::to_rabbit_trading_err(long_bridge_err))
    }
}

#[cfg(test)]
mod test_long_bridge_position {
    use rust_decimal_macros::dec;

    use super::LongBridgePosition;
    use crate::{broker::common::position_trait::Position, model::currency::Currency};

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_account_balance() {
        let long_bridge_position = LongBridgePosition::new().await;

        let account_balance_result = long_bridge_position.account_balance().await;
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
        let long_bridge_position = LongBridgePosition::new().await;

        let positions_result = long_bridge_position.positions().await;
        assert!(positions_result.is_ok());
        positions_result.unwrap().iter().for_each(|position| {
            assert!(position.cost_price >= dec!(0));
            assert!(position.available_quantity >= 0);
            assert!(position.total_quantity >= 0);
        });
    }
}
