use async_trait::async_trait;
use std::{collections::HashMap, time::Duration};

use crate::metrics::common::registry::MetricRegistryTrait;

pub struct StatsDMetricRegistry {}

#[async_trait]
impl MetricRegistryTrait for StatsDMetricRegistry {
    async fn inc_counter(&self, _name: String, _tags: HashMap<String, String>, _times: i32) {
        todo!()
    }

    async fn timer(&self, _name: String, _tags: HashMap<String, String>, _duration: Duration) {
        todo!()
    }

    async fn gauge(&self, _name: String, _tags: HashMap<String, String>, _value: i32) {
        todo!()
    }
}
