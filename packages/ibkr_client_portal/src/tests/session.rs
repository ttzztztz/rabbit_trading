use crate::{
    client::IBClientPortal,
    tests::utils::{get_test_account, TEST_HOST},
};

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
