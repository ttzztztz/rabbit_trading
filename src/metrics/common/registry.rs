use async_trait::async_trait;
use std::{collections::HashMap, time::Duration};

#[async_trait]
pub trait MetricRegistryTrait: Send + Sync {
    async fn inc_counter(&self, name: String, tags: HashMap<String, String>, times: i64);
    async fn inc_counter_once(&self, name: String, tags: HashMap<String, String>) {
        const ONCE: i64 = 1i64;

        self.inc_counter(name, tags, ONCE).await
    }

    async fn timer(&self, name: String, tags: HashMap<String, String>, duration: Duration);
    async fn gauge(&self, name: String, tags: HashMap<String, String>, value: String);

    // todo: adding more metrics here
}
