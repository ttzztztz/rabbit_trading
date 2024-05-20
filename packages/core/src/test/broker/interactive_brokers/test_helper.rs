use dotenv::dotenv;
use std::env;

use crate::model::common::types::ConfigMap;

pub(super) const ENV_KEY_TEST_ACCOUNT: &'static str = "IBKR_TEST_ACCOUNT";
pub(super) const ENV_VALUE_DEFAULT_TEST_ACCOUNT: &'static str = "0";

pub(super) fn get_test_account() -> String {
    dotenv().unwrap();
    env::var(ENV_KEY_TEST_ACCOUNT).unwrap_or(ENV_VALUE_DEFAULT_TEST_ACCOUNT.to_owned())
}

pub(super) fn get_config_map() -> ConfigMap {
    ConfigMap::from([
        ("ibkr.cp.yaml.path".to_owned(), "./ib.yaml".to_owned()),
        ("ibkr.cp.host".to_owned(), "localhost:5000".to_owned()),
        ("ibkr.cp.ssl".to_owned(), "false".to_owned()),
        ("ibkr.cp.account".to_owned(), get_test_account()),
        ("ibkr.cp.max.reply.count".to_owned(), "5".to_owned()),
    ])
}
