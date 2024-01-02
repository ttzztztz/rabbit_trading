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
        if let Err(err) = self
            .client
            .incr_by_value(&name, times, Self::transform_tags(tags))
        {
            log::error!("Error when inc_counter for name={}, {}", name, err);
        }
    }

    async fn timer(&self, name: String, tags: HashMap<String, String>, duration: Duration) {
        if let Err(err) = self.client.timing(
            &name,
            duration.as_millis() as i64,
            Self::transform_tags(tags),
        ) {
            log::error!("Error when timer for name={}, {}", name, err);
        }
    }

    async fn gauge(&self, name: String, tags: HashMap<String, String>, value: String) {
        if let Err(err) = self
            .client
            .gauge(&name, value.to_string(), Self::transform_tags(tags))
        {
            log::error!("Error when gauge for name={}, {}", name, err);
        }
    }
}
