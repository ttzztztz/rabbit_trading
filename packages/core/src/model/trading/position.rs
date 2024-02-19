use rust_decimal::Decimal;
use super::{currency::Currency, symbol::Symbol};

pub type PositionList = Vec<Position>;

pub struct Position {
    pub symbol: Symbol,
    pub currency: Currency,
    pub cost_price: Decimal,
    pub total_quantity: Decimal,
    pub available_quantity: Decimal,
}
