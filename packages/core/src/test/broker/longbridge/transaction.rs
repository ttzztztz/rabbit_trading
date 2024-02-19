use rust_decimal_macros::dec;

use crate::{
    broker::{
        common::transaction::TransactionTrait, longbridge::transaction::LongBridgeTransaction,
    },
    model::{common::types::ConfigMap, trading::currency::Currency},
};

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_account_balance() {
    let longbridge_transaction = LongBridgeTransaction::new(ConfigMap::new()).await;

    let account_balance_result = longbridge_transaction.account_balance().await;
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
    let longbridge_transaction = LongBridgeTransaction::new(ConfigMap::new()).await;

    let positions_result = longbridge_transaction.positions().await;
    assert!(positions_result.is_ok());
    positions_result.unwrap().iter().for_each(|position| {
        assert!(position.cost_price >= dec!(0));
        assert!(position.available_quantity >= dec!(0));
        assert!(position.total_quantity >= dec!(0));
    });
}
