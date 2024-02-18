use serde_json::Value;
use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::scanner::{HmdsScannerFilter, RunScannerBetaRequest, ScannerRunRequest},
    tests::utils::{get_test_account, TEST_HOST},
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_scanner_parameters() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client.get_scanner_parameters().await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.filter_list.len() > 0);
    assert!(response.instrument_list.len() > 0);
    assert!(response.location_tree.len() > 0);
    assert!(response.scan_type_list.len() > 0);
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_run_scanner_beta() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .run_scanner_beta(RunScannerBetaRequest {
            instrument: Option::Some("BOND.GOVT".to_owned()),
            locations: Option::Some("BOND.GOVT.US".to_owned()),
            scan_code: Option::Some("FAR_MATURITY_DATE".to_owned()),
            sec_type: Option::Some("BOND".to_owned()),
            filters: Option::Some(vec![HmdsScannerFilter {
                code: Option::Some("bondValidNetBidOrAskOnly".to_owned()),
                value: Option::Some(Value::Bool(true)),
            }]),
        })
        .await;
    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_scanner_run() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let response_result = ib_cp_client
        .scanner_run(ScannerRunRequest {
            instrument: Option::Some("STK".to_owned()),
            _type: Option::Some("MOST_ACTIVE_USD".to_owned()),
            location: Option::Some("STK.US.MAJOR".to_owned()),
            filter: Option::Some(vec![]),
        })
        .await;
    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.contracts.len() > 0);
}
