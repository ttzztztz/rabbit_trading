use crate::broker::longbridge::broker::LongBridgeBroker;
use crate::model::trading::{currency::Currency, market::Market, symbol::Symbol};

#[test]
fn test_to_currency() {
    assert!(LongBridgeBroker::to_currency("JPY").is_err());

    assert!(matches!(
        LongBridgeBroker::to_currency("CNH"),
        Ok(Currency::CNH),
    ));
    assert!(matches!(
        LongBridgeBroker::to_currency("USD"),
        Ok(Currency::USD),
    ));
    assert!(matches!(
        LongBridgeBroker::to_currency("HKD"),
        Ok(Currency::HKD),
    ));
}

#[test]
fn test_to_market() {
    assert!(LongBridgeBroker::to_market("JP").is_err());
    assert!(matches!(LongBridgeBroker::to_market("CN"), Ok(Market::CN)));
    assert!(matches!(LongBridgeBroker::to_market("US"), Ok(Market::US)));
    assert!(matches!(LongBridgeBroker::to_market("HK"), Ok(Market::HK)));
}

#[test]
fn test_to_symbol() {
    assert!(LongBridgeBroker::to_symbol("8316.JP").is_err());
    let symbol1 = Symbol {
        market: Market::US,
        identifier: "META".to_owned(),
    };
    let symbol2 = Symbol {
        market: Market::HK,
        identifier: "0700".to_owned(),
    };

    assert!(matches!(
        LongBridgeBroker::to_symbol("META.US"),
        Ok(res) if res == symbol1,
    ));
    assert!(matches!(
        LongBridgeBroker::to_symbol("0700.HK"),
        Ok(res) if res == symbol2,
    ));
}
