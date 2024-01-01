use super::common::strategy::{StrategyContext, StrategyTrait};
use crate::{
    model::common::error::Error, strategy::example::print_live_price::PrintLivePriceStrategy,
};

pub fn get_strategy_instance(
    identifier: String,
    strategy_context: StrategyContext,
) -> Result<Box<dyn StrategyTrait>, Error> {
    const IDENTIFIER_NOT_MATCHED_ERROR_CODE: &'static str = "IDENTIFIER_NOT_MATCHED";

    match identifier {
        identifier if identifier == PrintLivePriceStrategy::get_identifier() => Result::Ok(
            Box::new(PrintLivePriceStrategy::new(strategy_context)),
        ),

        _ => Result::Err(Error {
            code: IDENTIFIER_NOT_MATCHED_ERROR_CODE.to_owned(),
            message: format!("PersistentKV: {}", identifier),
        }),
    }
}
