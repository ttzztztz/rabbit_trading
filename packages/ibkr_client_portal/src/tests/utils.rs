use dotenv::dotenv;
use std::env;

pub(super) const ENV_KEY_TEST_ACCOUNT: &'static str = "IBKR_TEST_ACCOUNT";
pub(super) const TEST_ACCOUNT: &'static str = "0";
pub(super) const TEST_HOST: &'static str = "localhost:5000";
pub(super) const CONTRACT_ID_QQQ: i64 = 320227571;
pub(super) const CONTRACT_ID_AAPL: i64 = 265598;

pub(super) fn get_test_account() -> String {
    dotenv().unwrap();
    env::var(ENV_KEY_TEST_ACCOUNT).unwrap_or(TEST_ACCOUNT.to_owned())
}
