use async_trait::async_trait;
use std::{collections::HashMap, time::Duration};

use crate::metrics::common::registry::MetricRegistryTrait;

pub struct NoOpMetricRegistry {}

impl NoOpMetricRegistry {
    fn format_tags(tags: HashMap<String, String>) -> String {
        let pairs = tags
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join(",");

        format!("{{{}}}", pairs)
    }
}

#[async_trait]
impl MetricRegistryTrait for NoOpMetricRegistry {
    async fn inc_counter(&self, name: String, tags: HashMap<String, String>, times: i64) {
        log::warn!(
            "inc_counter::metrics={}, tags={}, times={}",
            name,
            Self::format_tags(tags),
            times,
        );
    }

    async fn timer(&self, name: String, tags: HashMap<String, String>, duration: Duration) {
        log::warn!(
            "timer::metrics={}, tags={}, duration={}ms",
            name,
            Self::format_tags(tags),
            duration.as_millis(),
        );
    }

    async fn gauge(&self, name: String, tags: HashMap<String, String>, value: String) {
        log::warn!(
            "gauge::metrics={}, tags={}, value={}",
            name,
            Self::format_tags(tags),
            value,
        );
    }
}
