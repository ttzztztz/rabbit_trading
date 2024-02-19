use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::{
        contract::{
            GetContractDetailRequest, GetContractRulesRequest, GetIBAlgorithmParametersRequest,
            GetInfoAndRulesByConIdRequest, GetSecurityDefinitionByConIdRequest,
            GetSecurityTradingScheduleRequest, GetStocksBySymbolRequest, SearchForSecurityRequest,
        },
        definition::AssetClass,
    },
    test::utils::{get_test_account, CONTRACT_ID_QQQ, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_security_definition_by_contract_id() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);

    let response_result = ib_cp_client
        .get_security_definition_by_contract_id(GetSecurityDefinitionByConIdRequest {
            conid_list: vec![CONTRACT_ID_QQQ],
        })
        .await;
    assert!(response_result.is_ok());
}

// todo: test get_futures_by_symbol

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
    assert_eq!("QQQ", response.symbol);
    assert!(response.valid_exchanges.len() > 0);
}

// todo: test get_contract_details_of_futures_options_warrants_cash_cfds, get_security_strikes, get_supported_algorithms_by_contract

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_search_for_security() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);

    let response_result = ib_cp_client
        .search_for_security(SearchForSecurityRequest {
            symbol: "QQQ".to_owned(),
            is_name: true,
            sec_type: AssetClass::Stock,
        })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_security_trading_schedule() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);

    let response_result = ib_cp_client
        .get_security_trading_schedule(GetSecurityTradingScheduleRequest {
            asset_class: AssetClass::Stock,
            symbol: "QQQ".to_owned(),
            exchange: Option::None,
            exchange_filter: Option::None,
        })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_contract_rules() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);

    let response_result = ib_cp_client
        .get_contract_rules(GetContractRulesRequest {
            conid: CONTRACT_ID_QQQ,
            is_buy: true,
        })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_info_and_rules_by_conid() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);

    let response_result = ib_cp_client
        .get_info_and_rules_by_conid(GetInfoAndRulesByConIdRequest {
            conid: CONTRACT_ID_QQQ,
            is_buy: true,
        })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_supported_algorithms_by_contract() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);

    let response_result = ib_cp_client
        .get_supported_algorithms_by_contract(GetIBAlgorithmParametersRequest {
            conid: CONTRACT_ID_QQQ,
            algos: Option::None,
            add_description: Option::None,
            add_params: Option::None,
        })
        .await;
    assert!(response_result.is_ok());
}
