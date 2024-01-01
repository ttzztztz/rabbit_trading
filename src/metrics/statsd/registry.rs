use async_trait::async_trait;
use dogstatsd::Client;
use std::{collections::HashMap, time::Duration};

use crate::metrics::common::registry::MetricRegistryTrait;

pub struct StatsDMetricRegistry {
    client: Client,
}

impl StatsDMetricRegistry {
    pub fn new(client: Client) -> Self {
        StatsDMetricRegistry { client }
    }

    fn transform_tags(tags: HashMap<String, String>) -> Vec<String> {
        tags.into_iter()
            .map(|(key, value)| format!("{}:{}", key, value))
            .collect()
    }
}

#[async_trait]
impl MetricRegistryTrait for StatsDMetricRegistry {
    async fn inc_counter(&self, name: String, tags: HashMap<String, String>, times: i64) {
        self.client
            .incr_by_value(name, times, Self::transform_tags(tags))
            .unwrap(); // todo: handle unwrap here
    }

    async fn timer(&self, name: String, tags: HashMap<String, String>, duration: Duration) {
        self.client
            .timing(
                name,
                duration.as_millis() as i64,
                Self::transform_tags(tags),
            )
            .unwrap(); // todo: handle unwrap here
    }

    async fn gauge(&self, name: String, tags: HashMap<String, String>, value: String) {
        self.client
            .gauge(name, value.to_string(), Self::transform_tags(tags))
            .unwrap(); // todo: handle unwrap here
    }
}
