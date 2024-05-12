use crate::{
    broker::interactive_brokers::{config::IBConfig, symbol::IBSymbolHelper},
    model::trading::{market::Market, symbol::Symbol},
};

#[test]
fn test_new_ib_symbol_helper() {
    let config = IBConfig::new("./ib.yaml".to_owned()).unwrap();
    let ib_symbol_helper = IBSymbolHelper::new(config);

    assert_eq!(
        265598,
        ib_symbol_helper
            .get_conid(Symbol {
                market: Market::US,
                identifier: "AAPL".to_owned(),
            })
            .unwrap()
    );
}
