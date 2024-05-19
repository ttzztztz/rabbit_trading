use std::str::FromStr;

use crate::model::trading::market::Market;

#[test]
fn test_market_from_str() {
    assert!(Market::from_str("unknown").is_err());
    assert!(matches!(Market::from_str("CN"), Ok(Market::CN)));
    assert!(matches!(Market::from_str("US"), Ok(Market::US)));
    assert!(matches!(Market::from_str("HK"), Ok(Market::HK)));
}

#[test]
fn test_market_to_string() {
    assert_eq!("CN", Market::to_string(&Market::CN));
    assert_eq!("HK", Market::to_string(&Market::HK));
    assert_eq!("US", Market::to_string(&Market::US));
}
