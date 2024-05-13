use crate::{
    broker::interactive_brokers::{broker::InteractiveBrokersBroker, config::IBConfig},
    model::common::types::ConfigMap,
};

#[test]
fn test_new_ib_config() {
    let config_map = ConfigMap::from([(
        InteractiveBrokersBroker::CONFIG_KEY_YAML_PATH.to_owned(),
        "./ib.yaml".to_owned(),
    )]);
    let config = IBConfig::new(&config_map).unwrap();
    assert!(config.symbol_to_conid.len() > 0);
}
