use dotenv::dotenv;
use std::{env, vec};

use crate::{
    client::IBClientPortal,
    model::{
        contract_detail::GetContractDetailRequest, market_data::MarketDataRequest,
        position::GetPositionsRequest, stock_contract::GetStocksBySymbolRequest,
        tick_types::TickType,
    },
};

const ENV_KEY_TEST_ACCOUNT: &'static str = "IBKR_TEST_ACCOUNT";
const TEST_ACCOUNT: &'static str = "0";
const TEST_HOST: &'static str = "localhost:5000";
const CONTRACT_ID_QQQ: i64 = 320227571;

fn get_test_account() -> String {
    dotenv().unwrap();
    env::var(ENV_KEY_TEST_ACCOUNT).unwrap_or(TEST_ACCOUNT.to_owned())
}

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_tickle() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client.tickle().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.session.len() > 0);
    assert!(response.user_id > 0);
}

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_stocks_by_symbol() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_stocks_by_symbol(GetStocksBySymbolRequest {
            symbols: vec!["QQQ".to_owned()],
        })
        .await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    let response_stock_contract_info_option = response.get("QQQ");
    assert!(response_stock_contract_info_option.is_some());
    let response_stock_contract_info = response_stock_contract_info_option.unwrap();
    assert!(response_stock_contract_info.len() > 0);
    let contract_info = &response_stock_contract_info[0];
    assert!(contract_info.contracts.len() > 0);
    let contract = &contract_info.contracts[0];
    assert!(contract.conid == CONTRACT_ID_QQQ);
    assert!(contract_info.name.starts_with("INVESCO QQQ"));
}

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_contract_detail() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_contract_detail(GetContractDetailRequest {
            conid: CONTRACT_ID_QQQ,
        })
        .await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    println!("{:?}", response);
    assert_eq!("QQQ", response.symbol);
    assert!(response.valid_exchanges.len() > 0);
}

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_positions() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .get_positions(GetPositionsRequest { page: 1 })
        .await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    response.into_iter().for_each(|position| {
        assert!(position.conid > 0);
    });
}

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_market_data() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .market_data(MarketDataRequest {
            conids: vec![CONTRACT_ID_QQQ.to_string()],
            since: Option::Some(1_705_230_000_000),
            fields: vec![TickType::LastPrice, TickType::Low, TickType::High],
        })
        .await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.len() > 0);
    let first_contract = response.first().unwrap();
    assert!(first_contract
        .get(TickType::LastPrice.to_string().as_str())
        .is_some());
    assert!(first_contract
        .get(TickType::Low.to_string().as_str())
        .is_some());
    assert!(first_contract
        .get(TickType::High.to_string().as_str())
        .is_some());
}

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_accounts_operation() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let test_account = get_test_account();

    let response_result = ib_cp_client.get_accounts().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.accounts.contains(&test_account));

    // todo: support testing switch_account
    // let response_result = ib_cp_client
    //     .switch_account(SwitchAccountRequest {
    //         account_id: test_account.clone(),
    //     })
    //     .await;
    // assert!(response_result.is_ok());
    // let response = response_result.unwrap();
    // assert!(response.set);
    // assert_eq!(test_account, response.account_id);
}
