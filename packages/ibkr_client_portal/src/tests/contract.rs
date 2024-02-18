use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::contract::{GetContractDetailRequest, GetStocksBySymbolRequest},
    tests::utils::{get_test_account, CONTRACT_ID_QQQ, TEST_HOST},
};

#[tokio::test]
#[serial]
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
#[serial]
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
