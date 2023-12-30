use std::time::SystemTime;

pub fn get_now_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64
}

#[cfg(test)]
mod test_utils_time {
    use crate::utils::time::get_now_unix_timestamp;

    #[test]
    fn test_get_now_unix_timestamp() {
        assert!(get_now_unix_timestamp() > 0u64);
        assert!(get_now_unix_timestamp() > 1_700_000_000u64);
    }
}
