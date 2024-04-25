use anyhow::{anyhow, Error};

use super::common::strategy::{StrategyContext, StrategyTrait};

#[cfg(feature = "strategy__example")]
use super::example::{grid_trading::GridTradingStrategy, print_live_price::PrintLivePriceStrategy};

pub fn get_strategy_instance(
    identifier: String,
    strategy_context: StrategyContext,
) -> Result<Box<dyn StrategyTrait>, Error> {
    match identifier {
        #[cfg(feature = "strategy__example")]
        identifier if identifier == PrintLivePriceStrategy::get_identifier() => {
            Result::Ok(Box::new(PrintLivePriceStrategy::new(strategy_context)))
        }

        #[cfg(feature = "strategy__example")]
        identifier if identifier == GridTradingStrategy::get_identifier() => {
            Result::Ok(Box::new(GridTradingStrategy::new(strategy_context)))
        }

        _ => Result::Err(anyhow!("IDENTIFIER_NOT_MATCHED Strategy: {}", identifier)),
    }
}
