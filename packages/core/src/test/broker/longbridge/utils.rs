use crate::broker::longbridge::broker::LongBridgeBroker;
use crate::model::trading::{currency::Currency, market::Market, symbol::Symbol};

#[test]
fn test_to_currency() {
    assert!(LongBridgeBroker::to_currency("JPY").is_err());
    assert_eq!(
        Result::Ok(Currency::CNH),
        LongBridgeBroker::to_currency("CNH")
    );
    assert_eq!(
        Result::Ok(Currency::USD),
        LongBridgeBroker::to_currency("USD")
    );
    assert_eq!(
        Result::Ok(Currency::HKD),
        LongBridgeBroker::to_currency("HKD")
    );
}

#[test]
fn test_to_market() {
    assert!(LongBridgeBroker::to_market("JP").is_err());
    assert_eq!(Result::Ok(Market::CN), LongBridgeBroker::to_market("CN"));
    assert_eq!(Result::Ok(Market::US), LongBridgeBroker::to_market("US"));
    assert_eq!(Result::Ok(Market::HK), LongBridgeBroker::to_market("HK"));
}

#[test]
fn test_to_symbol() {
    assert!(LongBridgeBroker::to_symbol("8316.JP").is_err());
    assert_eq!(
        Result::Ok(Symbol {
            market: Market::US,
            identifier: "META".to_owned(),
        }),
        LongBridgeBroker::to_symbol("META.US")
    );
    assert_eq!(
        Result::Ok(Symbol {
            market: Market::HK,
            identifier: "0700".to_owned(),
        }),
        LongBridgeBroker::to_symbol("0700.HK")
    );
}
