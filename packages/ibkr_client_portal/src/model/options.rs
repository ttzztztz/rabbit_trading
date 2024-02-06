use rust_decimal::Decimal;

use super::definition::AssetClass;

pub struct GetOptionsRequest {
    pub underlying_con_id: i64,
    pub sectype: AssetClass,
    pub month: Option<String>,
    pub exchange: Option<String>,
    pub strike: Option<Decimal>,
}
