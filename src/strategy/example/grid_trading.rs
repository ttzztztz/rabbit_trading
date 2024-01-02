use async_trait::async_trait;

use crate::{
    model::common::error::Error,
    strategy::common::strategy::{StrategyContext, StrategyTrait},
};

pub struct GridTradingStrategy {
    strategy_context: StrategyContext,
}

#[async_trait]
impl StrategyTrait for GridTradingStrategy {
    fn new(strategy_context: StrategyContext) -> Self {
        GridTradingStrategy { strategy_context }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "ExampleGridTradingStrategy";
        return IDENTIFIER.to_owned();
    }

    async fn start(&self) -> Result<(), Error> {
        todo!()
    }

    async fn stop(&self) -> Result<(), Error> {
        todo!() // todo: provide an approach to stop gracefully
    }
}
