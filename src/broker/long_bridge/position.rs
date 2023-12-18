use async_trait::async_trait;
use longbridge::TradeContext;

use super::broker::LongBridgeBroker;
use crate::{
    broker::common::position_trait::Position,
    model::{
        balance::{BalanceDetail, BalanceHashMap},
        currency::Currency,
        error::Error,
    },
};

pub struct LongBridgePosition {
    longbridge_context: TradeContext,
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
                    .map(|currency_balance| {
                        (
                            match currency_balance.currency.as_str() {
                                "HKD" => Option::Some(Currency::HKD),
                                "USD" => Option::Some(Currency::USD),
                                "CNH" => Option::Some(Currency::CNH),
                                _ => Option::None,
                            },
                            BalanceDetail {
                                total_cash: currency_balance.total_cash,
                                net_assets: currency_balance.net_assets,
                                margin_call: currency_balance.margin_call,
                                init_margin: currency_balance.init_margin,
                                maintenance_margin: currency_balance.maintenance_margin,
                            },
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
}
