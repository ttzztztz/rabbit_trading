use std::collections::HashMap;

use crate::persistent_kv::{common::store::PersistentKVStoreTrait, memory::store::MemoryKVStore};

#[tokio::test]
async fn test_operations_on_memory_kv_store() {
    const MAP_KEY_1: &str = "key_1";
    const MAP_VALUE_1: &str = "114514";
    const MAP_KEY_2: &str = "key_2";

    let kv_store: Box<dyn PersistentKVStoreTrait> =
        Box::new(MemoryKVStore::new(HashMap::new()).await);

    assert!(kv_store.read(MAP_KEY_1.to_owned()).await.is_err());
    assert!(kv_store.read(MAP_KEY_2.to_owned()).await.is_err());
    assert!(kv_store
        .write(MAP_KEY_1.to_owned(), MAP_VALUE_1.as_bytes().to_owned())
        .await
        .is_ok());
    assert!(kv_store.read(MAP_KEY_1.to_owned()).await.is_ok());
    assert!(kv_store.read(MAP_KEY_2.to_owned()).await.is_err());
}
