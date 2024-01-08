use async_trait::async_trait;

use crate::{broker::common::heartbeat::HeartbeatTrait, model::common::error::Error};

pub struct InteractiveBrokersHeartbeat {}

#[async_trait]
impl HeartbeatTrait for InteractiveBrokersHeartbeat {
    async fn new() -> Self {
        InteractiveBrokersHeartbeat {}
    }

    async fn start(&self) -> Result<(), Error> {
        todo!()
    }

    async fn stop(&self) -> Result<(), Error> {
        todo!()
    }
}
