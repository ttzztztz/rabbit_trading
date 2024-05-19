use std::str::FromStr;

use crate::model::trading::{market::Market, symbol::Symbol};

#[test]
fn test_symbol_from_str() {
    assert!(Symbol::from_str("8316.JP").is_err());
    let symbol1 = Symbol {
        market: Market::US,
        identifier: "META".to_owned(),
    };
    let symbol2 = Symbol {
        market: Market::HK,
        identifier: "0700".to_owned(),
    };

    assert!(matches!(
        Symbol::from_str("META.US"),
        Ok(res) if res == symbol1,
    ));
    assert!(matches!(
        Symbol::from_str("0700.HK"),
        Ok(res) if res == symbol2,
    ));
}

#[test]
fn test_symbol_to_string() {
    assert_eq!(
        "META.US",
        Symbol {
            market: Market::US,
            identifier: "META".to_owned(),
        }
        .to_string()
    );
}
