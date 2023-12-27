use async_trait::async_trait;

use crate::broker::common::info::InfoInterceptorTrait;

pub struct PodInfoInterceptor {}

#[async_trait]
impl InfoInterceptorTrait for PodInfoInterceptor {}
