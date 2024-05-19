use std::str::FromStr;

use crate::model::trading::currency::Currency;

#[test]
fn test_currency_from_str() {
    assert!(Currency::from_str("JPY").is_err());

    assert!(matches!(Currency::from_str("CNH"), Ok(Currency::CNH),));
    assert!(matches!(Currency::from_str("USD"), Ok(Currency::USD),));
    assert!(matches!(Currency::from_str("HKD"), Ok(Currency::HKD),));
}
