use rust_decimal::Decimal;
use std::collections::HashMap;

use super::currency::Currency;

pub type BalanceHashMap = HashMap<Currency, BalanceDetail>;

pub struct BalanceDetail {
    pub total_cash: Decimal,
    pub net_assets: Decimal,
    pub margin_call: Decimal,
    pub init_margin: Decimal,
    pub maintenance_margin: Decimal,
}
