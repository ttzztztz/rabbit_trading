use serial_test::serial;

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_connect_to_websocket() {
    todo!()
}
