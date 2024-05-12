use crate::broker::interactive_brokers::config::IBConfig;

#[test]
fn test_new_ib_config() {
    let config = IBConfig::new("./ib.yaml".to_owned()).unwrap();
    assert!(config.symbol_to_conid.len() > 0);
}
