use crate::utils::time::get_now_unix_timestamp;

#[test]
fn test_get_now_unix_timestamp() {
    assert!(get_now_unix_timestamp() > 0u64);
    assert!(get_now_unix_timestamp() > 1_700_000_000u64);
}
