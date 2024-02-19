use dotenv::dotenv;
use serial_test::serial;
use std::env;

use crate::{
    client::IBClientPortal,
    model::{
        definition::TickType,
        web_socket::{Subscription, SubscriptionType},
    },
};

const ENV_KEY_TEST_ACCOUNT: &'static str = "IBKR_TEST_ACCOUNT";
const TEST_ACCOUNT: &'static str = "0";
const TEST_HOST: &'static str = "localhost:5000";
const CONID_QQQ: i64 = 320227571;

fn get_test_account() -> String {
    dotenv().unwrap();
    env::var(ENV_KEY_TEST_ACCOUNT).unwrap_or(TEST_ACCOUNT.to_owned())
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_connect_to_websocket() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    ib_cp_client
        .connect_to_websocket(
            vec![Subscription {
                sub_type: SubscriptionType::QuoteData {
                    tick_types: vec![
                        // TickType::BidPrice,
                        // TickType::AskPrice,
                        // TickType::AskSize,
                        // TickType::BidSize,
                        TickType::LastPrice,
                        TickType::ChangePct,
                    ],
                    conid: 265598,
                },
                exchange: Option::None,
            }],
            |msg| println!("<- {}", msg),
        )
        .await;
}
