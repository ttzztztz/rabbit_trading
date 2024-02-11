use crate::{
    client::IBClientPortal,
    tests::utils::{get_test_account, TEST_HOST},
};

#[tokio::test]
#[cfg_attr(feature = "ci", ignore)]
async fn test_accounts_operation() {
    let ib_cp_client = IBClientPortal::new(get_test_account(), TEST_HOST.to_owned(), false);
    let test_account = get_test_account();

    let response_result = ib_cp_client.get_brokerage_accounts().await;
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
