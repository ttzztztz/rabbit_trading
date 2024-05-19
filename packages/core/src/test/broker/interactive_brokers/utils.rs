use rust_decimal_macros::dec;

use crate::{
    broker::interactive_brokers::broker::InteractiveBrokersBroker,
    model::trading::currency::Currency,
};

#[test]
fn test_depth_size_to_volume() {
    assert_eq!(
        dec!(114514),
        InteractiveBrokersBroker::depth_size_to_volume(Option::Some("114,514".to_owned())).unwrap(),
    );
    assert!(
        InteractiveBrokersBroker::depth_size_to_volume(Option::Some("114,invalid".to_owned()))
            .is_err(),
    );
    assert!(InteractiveBrokersBroker::depth_size_to_volume(Option::None).is_err());
}

#[test]
fn test_parse_currency_from_optional_string() {
    assert_eq!(
        Currency::USD,
        InteractiveBrokersBroker::parse_currency_from_optional_string(Option::Some(
            "USD".to_owned()
        ))
        .unwrap()
    );
    assert!(InteractiveBrokersBroker::parse_currency_from_optional_string(Option::None).is_err());
    assert!(
        InteractiveBrokersBroker::parse_currency_from_optional_string(Option::Some(
            "invalid".to_owned()
        ))
        .is_err(),
    );
}

#[test]
fn test_parse_last_price() {
    assert!(InteractiveBrokersBroker::parse_last_price(Option::None).is_err());
    assert!(
        InteractiveBrokersBroker::parse_last_price(Option::Some("invalid".to_owned())).is_err(),
    );
    assert_eq!(
        dec!(1145.14),
        InteractiveBrokersBroker::parse_last_price(Option::Some("1145.14".to_owned())).unwrap(),
    );
}
