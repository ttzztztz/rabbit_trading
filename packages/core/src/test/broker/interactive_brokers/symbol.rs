use crate::{
    broker::interactive_brokers::{
        broker::InteractiveBrokersBroker, config::IBConfig, symbol::IBSymbolHelper,
    },
    model::{
        common::types::ConfigMap,
        trading::{market::Market, symbol::Symbol},
    },
};

#[test]
fn test_new_ib_symbol_helper() {
    let config_map = ConfigMap::from([(
        InteractiveBrokersBroker::CONFIG_KEY_YAML_PATH.to_owned(),
        "./ib.yaml".to_owned(),
    )]);
    let config = IBConfig::new(&config_map).unwrap();
    let ib_symbol_helper = IBSymbolHelper::new(config);

    assert_eq!(
        265598,
        ib_symbol_helper
            .get_conid(&Symbol {
                market: Market::US,
                identifier: "AAPL".to_owned(),
            })
            .unwrap()
    );
}
