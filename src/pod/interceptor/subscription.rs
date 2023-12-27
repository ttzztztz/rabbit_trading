use async_trait::async_trait;

use crate::broker::common::subscription::SubscriptionInterceptorTrait;

pub struct PodSubscriptionInterceptor {}

#[async_trait]
impl SubscriptionInterceptorTrait for PodSubscriptionInterceptor {}
